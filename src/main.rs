mod app;
mod config;
mod git;
mod storage;
mod ui;

use anyhow::Result;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    // Check if we're running in a proper terminal
    if !atty::is(atty::Stream::Stdout) {
        eprintln!("Error: This application requires a proper terminal to run.");
        eprintln!("Please run this application from a terminal emulator like:");
        eprintln!("- Terminal.app on macOS");
        eprintln!("- Command Prompt, PowerShell, or Windows Terminal on Windows");
        eprintln!("- Any terminal emulator on Linux");
        eprintln!("\nYou cannot run this TUI application through IDEs or non-terminal environments.");
        std::process::exit(1);
    }

    let mut app = App::new().await?;
    app.run().await
}