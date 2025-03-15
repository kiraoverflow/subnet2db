use std::{thread, io::{self, Write}};
pub fn display_progress_bar(current: usize, total: usize) {
    let progress = (current as f64 / total as f64) * 100.0;
    
    let bar_width = 50;
    let filled_width = (progress / 100.0 * bar_width as f64).round() as usize;
    let bar = "=".repeat(filled_width) + &" ".repeat(bar_width - filled_width);

    // Print the progress bar and percentage
    print!("\r[{}] {:.2}%", bar, progress);
    io::stdout().flush().unwrap();  // Ensure the output is printed immediately
}
