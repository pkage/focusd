mod messages;

use clap::Clap;
mod client;

#[derive(Clap)]
#[clap(version="0.0.1", author="Patrick Kage (patrick@ka.ge)", about="Automatically manage /etc/hosts.txt to lock out distracting websites for a finite period.")]
struct Opts {

    #[clap(subcommand)]
    subcmd: SubCommand
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name="daemon", version="0.0.1", about="Run the focusd daemon")]
    Daemon(Daemon),

    #[clap(name="client", version="0.0.1", about="Connect to the focusd daemon")]
    Client(Client)
}

#[derive(Clap)]
struct Daemon {
    #[clap(short='s', long="socket", default_value="/tmp/focusd.sock", about="Socket to listen on")]
    socket: String,

    #[clap(default_value="~/.config/focusd/focusdrc")]
    config: String
}

#[derive(Clap)]
struct Client {
    #[clap(short='s', long="socket", default_value="/tmp/focusd.sock", about="Socket to use")]
    socket: String,

    #[clap(subcommand)]
    subcmd: ClientCommand
}

#[derive(Clap)]
enum ClientCommand {
    #[clap(name="ping", about="Check if the daemon is running")]
    Ping(ClientCommandBase),

    #[clap(name="remaining", about="Check if the daemon is running")]
    Remaining(ClientCommandBase),

    #[clap(name="start", about="Start blocking the files.")]
    Start(ClientCommandStart),

    #[clap(name="repl", about="(debug) connect a repl to the server")]
    Repl(ClientCommandBase)
}

#[derive(Clap)]
struct ClientCommandBase {}

#[derive(Clap)]
struct ClientCommandStart {
    #[clap(name="length", about="Length of time to run the block (e.g. 1h25m30s)")]
    length: String,
}


fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Daemon(d) => {
            println!("Starting daemon on socket {}", d.socket);
        },
        SubCommand::Client(c) => {
            println!("Starting client on socket {}", c.socket);

            let mut client = match client::FocusClient::new(c.socket) {
                Ok(c) => c,
                Err(_) => {
                    println!("error initializing!");
                    panic!("PANIC");
                }
            };

            match c.subcmd {
                ClientCommand::Ping(_)      => println!("\tselected: ping"),
                ClientCommand::Start(_)     => println!("\tselected: start"),
                ClientCommand::Remaining(_) => println!("\tselected: remaining"),
                ClientCommand::Repl(_)      => println!("\tselected: repl")
            };

            client.ping();
            client.destroy();
        }
    }
}
