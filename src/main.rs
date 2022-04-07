use actix_files as fs;
use actix_web::{middleware::Logger, App, HttpServer};
use log::{debug, info, warn};
use std::env;

const PORT_KEY: &str = "PORT";
const DEFAULT_PORT: u16 = 8080;

/// Load port number from environmental variables.
///
/// # Returns
///
/// Port number. If the number is not defined in env or invalid string, use default number.
fn load_port() -> u16 {
    let default_port_msg = format!("Default port ({}) will be used.", DEFAULT_PORT);

    env::var(PORT_KEY)
        .unwrap_or_else(|_| {
            warn!(
                "\"{}\" is not defined in environment variables. {}",
                PORT_KEY, default_port_msg
            );
            DEFAULT_PORT.to_string()
        })
        .parse::<u16>()
        .unwrap_or_else(|_| {
            warn!("The port number is invalid. {}", default_port_msg);
            DEFAULT_PORT
        })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("The server is starting!");
    debug!("build server");
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(fs::Files::new("/", "./static").show_files_listing())
    });

    debug!("bind port");
    let port = load_port();
    server.bind(("0.0.0.0", port))?.run().await?;

    info!("The server has stopped!");

    Ok(())
}
