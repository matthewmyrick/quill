mod app;
mod git;
mod storage;
mod ui;

use anyhow::Result;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new().await?;
    app.run().await
}