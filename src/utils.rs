use log::warn;
use std::env;

const PORT_KEY: &str = "PORT";
const DEFAULT_PORT: u16 = 8080;

/// Load port number from environmental variables.
///
/// # Returns
///
/// Port number. If the number is not defined in env or invalid string, use default number.
pub fn load_port() -> u16 {
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
