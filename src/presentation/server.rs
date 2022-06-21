use super::{modules::Modules, responder};

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use log::{debug, info, warn};

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("The server is starting!");

    debug!("build server");
    let server = HttpServer::new(|| {
        App::new()
            .app_data(Data::new(Modules::default()))
            .wrap(Logger::default())
            .service(responder::list_posts)
            .service(responder::show_post)
    });
    debug!("bind port");
    let port = load_port();

    server.bind(("0.0.0.0", port))?.run().await?;

    info!("The server has stopped!");

    Ok(())
}

const PORT_KEY: &str = "PORT";
const DEFAULT_PORT: u16 = 8080;

fn load_port() -> u16 {
    let default_port_msg = format!("Default port ({}) will be used.", DEFAULT_PORT);

    std::env::var(PORT_KEY)
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
