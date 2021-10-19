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
    Daemon,

    #[clap(name="cleanup", about="Clean up the sockets")]
    Cleanup,

    #[clap(name="debug",   about="debug")]
    Debug,

    #[clap(name="ping", about="Check if the daemon is running")]
    Ping,

    #[clap(name="remaining", about="Check if the daemon is running")]
    Remaining(ClientCommandRemaining),

    #[clap(name="start", about="Start blocking the files.")]
    Start(ClientCommandStart),

    #[clap(name="halt", about="Halt the server")]
    Halt,

}

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

fn get_client(config: &config::FocusConfig) -> client::FocusClient {
    let client = match client::FocusClient::new(&config) {
        Ok(c) => c,
        Err(e) => {
            match e {
                // client::FocusClientError::TimedOut => println!("{}", "server timed out!"),
                // client::FocusClientError::ServerError => println!("{}", "server errored out!".red()),
                client::FocusClientError::NoConnection => println!("{}", "not running".red()),
            }
            std::process::exit(0);
        }
    };
    return client;
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
        SubCommand::Daemon => {
            server::FocusServer::cleanup(&config);

            let mut daemon = match server::FocusServer::new(&config) {
                Ok(d) => d,
                Err(e) => {
                    match e {
                        server::FocusServerError::AlreadyRunning => println!("{}", "server already running!".red()),
                        // server::FocusServerError::NoPermissions => println!("{}", "server should be run as root".red())
                    }
                    return;
                }
            };

            daemon.listen();

            server::FocusServer::cleanup(&config);
        },
        SubCommand::Ping => get_client(&config).ping(),
        SubCommand::Halt => get_client(&config).halt(),
        SubCommand::Remaining(r) => {
            get_client(&config).remaining(r.raw, r.no_distract);
        },
        SubCommand::Start(s) => {
            get_client(&config).start(s.length);
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
