use modcli::output::print;
use modcli::output::progress::{ProgressBar, ProgressStyle};
use std::time::Duration;

// Example: drive the progress bar from an actual process.
// Here we simulate downloading N chunks and call set_progress/tick
// based on real work rather than a fixed-duration auto-run.
fn main() {
    print::line("Real progress driven by work:");

    let total_chunks = 50;
    let mut bar = ProgressBar::new(total_chunks, ProgressStyle::default());
    bar.set_label("Downloading");

    // Simulate a workload providing chunks at variable timing
    let simulated_chunk_times_ms: Vec<u64> = (0..total_chunks)
        .map(|i| 10 + ((i * 13) % 25) as u64) // variable work time per chunk
        .collect();

    for (i, delay) in simulated_chunk_times_ms.into_iter().enumerate() {
        // ... do real work here (read chunk, write, hash, etc.)
        std::thread::sleep(Duration::from_millis(delay));

        // Update progress by actual number of chunks completed
        bar.set_progress(i + 1);
    }

    println!(" {}", bar.style.done_label);
}
