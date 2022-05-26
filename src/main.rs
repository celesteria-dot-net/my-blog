use my_blog::{responder, utils};

use actix_web::{middleware::Logger, App, HttpServer};
use log::{debug, info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("The server is starting!");

    debug!("build server");
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(responder::list_posts)
    });
    debug!("bind port");
    let port = utils::load_port();

    server.bind(("0.0.0.0", port))?.run().await?;

    info!("The server has stopped!");

    Ok(())
}
