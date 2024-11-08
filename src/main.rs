use tracing::info;

fn init_env() {
    std::env::set_var("RUST_LOG", "trace");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_env();
    tracing_subscriber::fmt::init();

    info!("Hello, World!");

    Ok(())
}
