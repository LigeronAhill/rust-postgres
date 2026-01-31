#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = isac::configuration::init()?;
    isac::logger::init(&config)?;
    tracing::info!("{config:#?}");
    let app = isac::App::build(&config).await?;
    app.run().await?;
    Ok(())
}
