use modcli::output::progress::{MultiProgress, ProgressStyle};
use std::time::Duration;

fn main() {
    let mut mp = MultiProgress::new();
    let i1 = mp.add_bar("Download A", 40, ProgressStyle::default());
    let i2 = mp.add_bar("Download B", 30, ProgressStyle::default());
    let i3 = mp.add_bar("Download C", 20, ProgressStyle::default());

    for step in 0..40 {
        if step < 40 {
            mp.set_progress(i1, step + 1);
        }
        if step < 30 {
            mp.set_progress(i2, (step + 1).min(30));
        }
        if step < 20 {
            mp.set_progress(i3, (step + 1).min(20));
        }
        mp.refresh();
        std::thread::sleep(Duration::from_millis(40));
    }

    mp.finish();
}
