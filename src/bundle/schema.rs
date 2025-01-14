use serde::Deserialize;

#[derive(Deserialize)]
pub struct Bundle {
    config: BundleConfig,
}

#[derive(Deserialize)]
struct BundleConfig {
    loglevel: String,
}
