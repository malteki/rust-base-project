use tracing::info;

use name::env_vars::*;

//

fn init_env() {
    std::env::set_var("RUST_LOG", "trace");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> anyhow::Result<()> {
    init_env();
    tracing_subscriber::fmt::init();
    info!("running {CARGO_PKG_NAME} v{CARGO_PKG_VERSION}");

    Ok(())
}
