//! Authorization runs on every dispatch path: direct, alias, nested, cached,
//! and async.

use modcli::command::Command;
use modcli::loader::CommandRegistry;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

struct Wipe(Arc<AtomicUsize>);

impl Command for Wipe {
    fn name(&self) -> &str {
        "wipe"
    }
    fn aliases(&self) -> &[&str] {
        &["w"]
    }
    fn required_caps(&self) -> &[&str] {
        &["admin"]
    }
    fn execute(&self, _args: &[String]) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

#[test]
fn policy_is_checked_on_every_call() {
    let runs = Arc::new(AtomicUsize::new(0));
    let mut reg = CommandRegistry::new();
    reg.register(Box::new(Wipe(Arc::clone(&runs))));
    reg.set_authorize_policy(|_cmd, _caps, args| {
        if args.iter().any(|a| a == "--dry-run") {
            Ok(())
        } else {
            Err("denied".into())
        }
    });
    for token in ["wipe", "w"] {
        assert!(reg.try_execute(token, &args(&["--force"])).is_err());
        assert!(reg.try_execute(token, &args(&["--dry-run"])).is_ok());
        // A second call for the same token must be checked again.
        assert!(reg.try_execute(token, &args(&["--force"])).is_err());
    }
    assert_eq!(runs.load(Ordering::SeqCst), 2);
}

#[test]
fn revoked_capability_takes_effect_at_once() {
    let runs = Arc::new(AtomicUsize::new(0));
    let mut reg = CommandRegistry::new();
    reg.register(Box::new(Wipe(Arc::clone(&runs))));
    reg.grant_cap("admin");
    assert!(reg.try_execute("wipe", &[]).is_ok());
    reg.revoke_cap("admin");
    assert!(reg.try_execute("wipe", &[]).is_err());
    reg.set_caps(["admin"]);
    assert!(reg.try_execute("w", &[]).is_ok());
    reg.set_caps(Vec::<String>::new());
    assert!(reg.try_execute("w", &[]).is_err());
    assert_eq!(runs.load(Ordering::SeqCst), 2);
}

#[test]
fn post_hook_runs_on_repeated_calls() {
    let posts = Arc::new(AtomicUsize::new(0));
    let p2 = Arc::clone(&posts);
    let mut reg = CommandRegistry::new();
    reg.register(Box::new(Wipe(Arc::new(AtomicUsize::new(0)))));
    reg.grant_cap("admin");
    reg.set_post_hook(move |_, _, _| {
        p2.fetch_add(1, Ordering::SeqCst);
    });
    for _ in 0..3 {
        reg.try_execute("wipe", &[]).unwrap();
    }
    assert_eq!(posts.load(Ordering::SeqCst), 3);
}

/// Registering a command that takes over a cached alias token routes to the
/// new command.
#[test]
fn register_replaces_cached_resolution() {
    struct Named(&'static str, Arc<Mutex<Vec<&'static str>>>);
    impl Command for Named {
        fn name(&self) -> &str {
            self.0
        }
        fn execute(&self, _args: &[String]) {
            self.1.lock().unwrap().push(self.0);
        }
    }
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut reg = CommandRegistry::new();
    reg.register(Box::new(Wipe(Arc::new(AtomicUsize::new(0)))));
    reg.grant_cap("admin");
    reg.try_execute("w", &[]).unwrap();
    reg.register(Box::new(Named("w", Arc::clone(&log))));
    reg.try_execute("w", &[]).unwrap();
    assert_eq!(*log.lock().unwrap(), ["w"]);
}

/// A command may dispatch another command through the registry.
#[test]
fn command_can_dispatch_again() {
    struct Outer(Arc<AtomicUsize>);
    impl Command for Outer {
        fn name(&self) -> &str {
            "outer"
        }
        fn execute(&self, _args: &[String]) {}
        fn execute_with(&self, args: &[String], reg: &CommandRegistry) {
            self.0.fetch_add(1, Ordering::SeqCst);
            if args.is_empty() {
                reg.try_execute("outer", &["inner".to_string()]).unwrap();
            }
        }
    }
    let n = Arc::new(AtomicUsize::new(0));
    let mut reg = CommandRegistry::new();
    reg.register(Box::new(Outer(Arc::clone(&n))));
    reg.try_execute("outer", &[]).unwrap();
    reg.try_execute("outer", &[]).unwrap();
    assert_eq!(n.load(Ordering::SeqCst), 4);
}

#[test]
fn prefix_routing_accepts_any_utf8() {
    let runs = Arc::new(AtomicUsize::new(0));
    let mut reg = CommandRegistry::new();
    reg.register(Box::new(Wipe(Arc::clone(&runs))));
    reg.grant_cap("admin");
    reg.set_prefix("t");
    for name in [
        "\u{e9}\u{e9}",
        "\u{e9}",
        "t\u{e9}",
        "\u{1f600}:wipe",
        "t:",
        "t",
    ] {
        assert!(reg.try_execute(name, &[]).is_err(), "{name}");
    }
    reg.try_execute("t:wipe", &[]).unwrap();
    reg.set_prefix("\u{e9}");
    for name in ["e", "ab", "\u{e9}", "\u{e9}:"] {
        assert!(reg.try_execute(name, &[]).is_err(), "{name}");
    }
    reg.try_execute("\u{e9}:wipe", &[]).unwrap();
    assert_eq!(runs.load(Ordering::SeqCst), 2);
}

#[cfg(feature = "internal-commands")]
#[test]
fn help_renders_non_ascii_text() {
    struct H;
    impl Command for H {
        fn name(&self) -> &str {
            "deploy"
        }
        fn help(&self) -> Option<&str> {
            Some("Deploy \u{2014} ship it to the caf\u{e9} \u{2022} **now**")
        }
        fn execute(&self, _: &[String]) {}
    }
    let mut reg = CommandRegistry::new();
    reg.register(Box::new(H));
    reg.try_execute("help", &args(&["deploy"])).unwrap();
    reg.try_execute("help", &[]).unwrap();
}

#[cfg(feature = "async")]
mod async_commands {
    use super::*;
    use modcli::command::AsyncCommand;
    use modcli::error::ModCliError;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll, Wake, Waker};

    struct DropDb {
        caps: &'static [&'static str],
        runs: Arc<AtomicUsize>,
    }

    impl AsyncCommand for DropDb {
        fn name(&self) -> &str {
            "drop-db"
        }
        fn required_caps(&self) -> &[&str] {
            self.caps
        }
        fn execute_async<'a>(
            &'a self,
            _args: &'a [String],
        ) -> Pin<Box<dyn Future<Output = Result<(), ModCliError>> + Send + 'a>> {
            self.runs.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { Ok(()) })
        }
    }

    struct NoopWake;
    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}
    }

    /// Polls a future that is ready at once.
    fn block_on<F: Future>(fut: F) -> F::Output {
        let waker = Waker::from(Arc::new(NoopWake));
        let mut cx = Context::from_waker(&waker);
        let mut fut = Box::pin(fut);
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => v,
            Poll::Pending => panic!("future was not ready"),
        }
    }

    #[test]
    fn policy_sees_the_real_async_command() {
        let runs = Arc::new(AtomicUsize::new(0));
        let seen = Arc::new(Mutex::new(String::new()));
        let s2 = Arc::clone(&seen);
        let mut reg = CommandRegistry::new();
        reg.register_async(Box::new(DropDb {
            caps: &[],
            runs: Arc::clone(&runs),
        }));
        reg.set_authorize_policy(move |cmd, _caps, _args| {
            *s2.lock().unwrap() = cmd.name().to_owned();
            if cmd.name() == "drop-db" {
                Err("no".into())
            } else {
                Ok(())
            }
        });
        let r = block_on(reg.try_execute_async("drop-db", &[]));
        assert_eq!(&*seen.lock().unwrap(), "drop-db");
        assert!(r.is_err());
        assert_eq!(runs.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn async_required_caps_are_checked() {
        let runs = Arc::new(AtomicUsize::new(0));
        let mut reg = CommandRegistry::new();
        reg.register_async(Box::new(DropDb {
            caps: &["admin"],
            runs: Arc::clone(&runs),
        }));
        assert!(block_on(reg.try_execute_async("drop-db", &[])).is_err());
        assert_eq!(runs.load(Ordering::SeqCst), 0);
        reg.grant_cap("admin");
        assert!(block_on(reg.try_execute_async("drop-db", &[])).is_ok());
        assert_eq!(runs.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn async_prefix_routing_accepts_any_utf8() {
        let reg = {
            let mut r = CommandRegistry::new();
            r.set_prefix("t");
            r
        };
        assert!(block_on(reg.try_execute_async("\u{e9}\u{e9}", &[])).is_err());
    }
}
