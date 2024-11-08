//  moudles

pub mod errors;

// constants

pub mod env_vars {
    pub const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

    pub const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub const CARGO_PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

    pub const CARGO_PKG_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
}
