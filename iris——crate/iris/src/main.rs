use clap::Parser;
use iris_lib::{
    connect::{ConnectionError, ConnectionManager},
    irc_connection::IrcConnection,
    irc_connection::*,
    types::SERVER_NAME,
};
use log::*;
use simplelog::*;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Parser)]
struct Arguments {
    #[clap(default_value = "127.0.0.1")]
    ip_address: IpAddr,

    #[clap(default_value = "6991")]
    port: u16,
}
// #[warn(clippy::let_unit_value)]
fn main() {
    let arguments = Arguments::parse();
    let _logging = SimpleLogger::init(LevelFilter::Info, Config::default());
    info!(
        "Launching {} at {}:{}",
        SERVER_NAME, arguments.ip_address, arguments.port
    );

    let mut connection_manager = ConnectionManager::launch(arguments.ip_address, arguments.port);
    let user_connections = Arc::new(Mutex::new(IrcConnection::new()));
    info! {"finish to build the manager"};
    thread::scope(|s| {
        loop {
            info! {"start to get the thread connections"};
            // This function call will block until a new client connects!
            let (mut conn_read, conn_write) = connection_manager.accept_new_connection();

            info!("New connection from {}", conn_read.id());
            let threads_for_user_connections = user_connections.clone();

            s.spawn(move || {
                let mut handler = MessageHandler::new(&threads_for_user_connections, conn_write);
                while !handler.has_quit() {
                    info!("Waiting for message...");
                    match conn_read.read_message() {
                        Ok(message) => {
                            handler.handle_parsed_message(message.clone());
                            info!("Received message: {}", message.clone());
                        }
                        Err(
                            ConnectionError::ConnectionLost | ConnectionError::ConnectionClosed,
                        ) => {
                            info!("Lost connection.");
                            break;
                        }
                        Err(_) => {
                            info!("Invalid message received... ignoring message.");
                            continue;
                        }
                    };
                }

                info!("Connection has closed...");
            });
        }
    });
}
