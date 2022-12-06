use anyhow::Result;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    match mma().await {
        Ok(_) => {}
        Err(e) => log::error!("Error: {}", e),
    }
}

async fn mma() -> Result<()> {
    Ok(())
}
