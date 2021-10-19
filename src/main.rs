use colored::*;
use clap::Clap;

mod hosts;
mod time;
mod config;
mod client;
mod messages;
mod common;
mod server;

#[derive(Clap)]
#[clap(version="0.0.1", author="Patrick Kage (patrick@ka.ge)", about="Automatically manage /etc/hosts to lock out distracting websites for a finite period.")]
struct Opts {
    #[clap(short='c', long="config", default_value="~/.config/focusd/focus.toml", about="The config file to use.")]
    config: String,

    #[clap(subcommand)]
    subcmd: SubCommand
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name="daemon",  version="0.0.1", about="Run the focusd daemon")]
    Daemon(Daemon),

    #[clap(name="client",  version="0.0.1", about="Connect to the focusd daemon")]
    Client(Client),

    #[clap(name="cleanup", version="0.0.1", about="Clean up the sockets")]
    Cleanup,

    #[clap(name="debug", version="0.0.1", about="debug")]
    Debug,

}


#[derive(Clap)]
struct Client {
    #[clap(subcommand)]
    subcmd: ClientCommand
}

#[derive(Clap)]
enum ClientCommand {
    #[clap(name="ping", about="Check if the daemon is running")]
    Ping(ClientCommandBase),

    #[clap(name="remaining", about="Check if the daemon is running")]
    Remaining(ClientCommandRemaining),

    #[clap(name="start", about="Start blocking the files.")]
    Start(ClientCommandStart),

    #[clap(name="halt", about="Halt the server")]
    Halt(ClientCommandBase)
}

#[derive(Clap)]
struct ClientCommandBase {}

#[derive(Clap)]
struct ClientCommandStart {
    #[clap(name="length", about="Length of time to run the block (e.g. 1h25m30s)")]
    length: String,
}

#[derive(Clap)]
struct ClientCommandRemaining {
    #[clap(long="raw", short='r', about="Leave the time in seconds")]
    raw: bool,
}

#[derive(Clap)]
struct Daemon {
}

fn main() {
    let opts: Opts = Opts::parse();

    // validate configuration
    let config = match config::read_config(&opts.config) {
        Ok(cfg) => cfg,
        Err(err) => {
            match err {
                config::FocusConfigError::ConfigMissing => println!("{}", "config file missing!".red()),
                config::FocusConfigError::ConfigInvalid => println!("{}", "config file invalid!")
            }
            return;
        }
    };

    match opts.subcmd {
        SubCommand::Daemon(_) => {
            let mut daemon = match server::FocusServer::new(&config) {
                Ok(d) => d,
                Err(e) => {
                    match e {
                        server::FocusServerError::AlreadyRunning => println!("{}", "server already running!".red()),
                        server::FocusServerError::NoPermissions => println!("{}", "server should be run as root".red())
                    }
                    return;
                }
            };

            daemon.listen();

            daemon.cleanup();
        },
        SubCommand::Client(c) => {
            let client = match client::FocusClient::new(&config) {
                Ok(c) => c,
                Err(e) => {
                    match e {
                        // client::FocusClientError::TimedOut => println!("{}", "server timed out!".red()),
                        // client::FocusClientError::ServerError => println!("{}", "server errored out!".red()),
                        client::FocusClientError::NoConnection => println!("{}", "server not running!".red()),
                    }
                    std::process::exit(1);
                }
            };

            match c.subcmd {
                ClientCommand::Ping(_)      => client.ping(),
                ClientCommand::Start(s)     => client.start(s.length),
                ClientCommand::Remaining(r) => client.remaining(r.raw),
                ClientCommand::Halt(_)      => client.halt()
            };

        },
        SubCommand::Cleanup => {
            common::file_remove_if_exists(&format!("{}.in", config.socket_file));
            common::file_remove_if_exists(&format!("{}.out", config.socket_file));
        },
        SubCommand::Debug => {
            // let out = common::hosts_remove(&"hosts".to_string()).unwrap();
            // println!("{}", out);
            // println!("config: {:?}", config.blocked);
            // time::parse_time_string(&"1h30m25s".to_string()).unwrap();
            // time::parse_time_string(&"1h30m".to_string()).unwrap();
            // time::parse_time_string(&"30m".to_string()).unwrap();
            // time::parse_time_string(&"30s".to_string()).unwrap();
            // let out = hosts::hosts_add(&"hosts".to_string(), &config.blocked).unwrap();
            // println!("{}", out);
            time::parse_time_string(&"1h30m25s".to_string()).unwrap();
            time::create_time_string(5425);
            time::parse_time_string(&"30m".to_string()).unwrap();
            time::create_time_string(1800);
            time::parse_time_string(&"30s".to_string()).unwrap();
            time::create_time_string(30);
        }
    }
}
