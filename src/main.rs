use colored::*;
use clap::Parser;

mod hosts;
mod time;
mod config;
mod client;
mod messages;
mod common;
mod server;

#[derive(Parser)]
#[clap(version="0.0.2", author="Patrick Kage (patrick@ka.ge)", about="Automatically manage /etc/hosts to lock out distracting websites for a finite period.")]
struct Opts {
    #[clap(short='c', long="config", default_value="~/.config/focusd/focus.toml", about="The config file to use.")]
    config: String,

    #[clap(subcommand)]
    subcmd: SubCommand
}

#[derive(Parser)]
enum SubCommand {
    #[clap(name="daemon",  about="Run the focusd daemon")]
    Daemon(Daemon),

    #[clap(name="client",  about="Connect to the focusd daemon")]
    Client(Client),

    #[clap(name="cleanup", about="Clean up the sockets")]
    Cleanup,

    #[clap(name="debug",   about="debug")]
    Debug,

}


#[derive(Parser)]
struct Client {
    #[clap(subcommand)]
    subcmd: ClientCommand
}

#[derive(Parser)]
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

#[derive(Parser)]
struct ClientCommandBase {}

#[derive(Parser)]
struct ClientCommandStart {
    #[clap(name="length", about="Length of time to run the block (e.g. 1h25m30s)")]
    length: String,
}

#[derive(Parser)]
struct ClientCommandRemaining {
    #[clap(long="raw", short='r', about="Leave the time in seconds")]
    raw: bool,

    #[clap(long="nodistract", short='n', about="Omit the seconds from the count")]
    no_distract: bool,
}

#[derive(Parser)]
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
            server::FocusServer::cleanup(&config);

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

            server::FocusServer::cleanup(&config);
        },
        SubCommand::Client(c) => {
            if !common::check_pid_file(&config.pid_file) {
                println!("{}", "server not running!".red());
                return;
            }
            let client = match client::FocusClient::new(&config) {
                Ok(c) => c,
                Err(e) => {
                    match e {
                        // client::FocusClientError::TimedOut => println!("{}", "server timed out!"),
                        // client::FocusClientError::ServerError => println!("{}", "server errored out!".red()),
                        client::FocusClientError::NoConnection => println!("{}", "server not running!".red()),
                    }
                    return;
                }
            };

            match c.subcmd {
                ClientCommand::Ping(_)      => client.ping(),
                ClientCommand::Start(s)     => client.start(s.length),
                ClientCommand::Remaining(r) => {client.remaining(r.raw, r.no_distract);},
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
