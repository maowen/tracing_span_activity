#![deny(rust_2018_idioms)]
#[path = "fmt/yak_shave.rs"]
mod yak_shave;

use tracing;

fn main() {
    use tracing_subscriber::{fmt, EnvFilter};
    fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let number_of_yaks = 3;
    tracing::debug!("preparing to shave {} yaks", number_of_yaks);

    let number_shaved = yak_shave::shave_all(number_of_yaks);

    tracing::debug!(
        message = "yak shaving completed.",
        all_yaks_shaved = number_shaved == number_of_yaks,
    );
}
