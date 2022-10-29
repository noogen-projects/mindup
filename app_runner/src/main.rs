use std::path::Path;

use laplace_server::{error::AppResult, settings::Settings};

#[actix_web::main]
async fn main() -> AppResult<()> {
    let settings = Settings::new(Path::new("app_runner").join("settings.toml")).expect("Settings should be configured");
    laplace_server::init_logger(&settings.log)?;

    laplace_server::run(settings).await
}
