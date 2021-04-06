use std::{io, path::Path};

use dapla_server::{actix_web, settings::Settings};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let settings = Settings::new(Path::new("app_runner").join("settings.toml")).expect("Settings should be configured");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(settings.log.level.to_string()));

    dapla_server::run(settings).await
}
