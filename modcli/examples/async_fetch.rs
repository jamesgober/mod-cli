// Run with: cargo run --example async_fetch --features async,tokio-runtime -- <url>

#[cfg(feature = "async")]
use modcli::command::AsyncCommand;
#[cfg(feature = "async")]
use modcli::error::ModCliError;
#[cfg(feature = "async")]
use modcli::loader::CommandRegistry;
#[cfg(feature = "async")]
use std::future::Future;
#[cfg(feature = "async")]
use std::pin::Pin;
#[cfg(feature = "async")]
use std::time::Duration;

#[cfg(feature = "async")]
struct Fetch;

#[cfg(feature = "async")]
impl AsyncCommand for Fetch {
    fn name(&self) -> &str { "fetch" }
    fn execute_async<'a>(
        &'a self,
        args: &'a [String],
    ) -> Pin<Box<dyn Future<Output = Result<(), ModCliError>> + Send + 'a>> {
        Box::pin(async move {
            let url = args
                .get(0)
                .cloned()
                .unwrap_or_else(|| "https://example.com".to_string());

            println!("Starting fetch: {url}");
            // Simulate async network work; replace with reqwest or your client of choice.
            tokio::time::sleep(Duration::from_millis(500)).await;
            println!("Done: {url}");
            Ok(())
        })
    }
}

#[cfg(all(feature = "async", feature = "tokio-runtime"))]
#[tokio::main]
async fn main() {
    let mut reg = CommandRegistry::new();
    reg.register_async(Box::new(Fetch));

    // Invoke: cargo run --example async_fetch --features async,tokio-runtime -- <url>
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = "fetch";
    if args.is_empty() { reg.execute_async(cmd, &[]).await; } else { reg.execute_async(cmd, &args).await; }
}

#[cfg(not(all(feature = "async", feature = "tokio-runtime")))]
fn main() {
    eprintln!("This example requires `--features async,tokio-runtime`");
}
