use clap::{Parser, Subcommand};
use tokio::sync::mpsc;
use util::parse_list;

mod port;
mod util;

#[derive(Debug, Parser)]
#[command(name = "rnetmap")]
#[command(about = "Network Mapper designed with simplicity and performance in mind", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(short_flag = 'p')]
    Port {
        #[arg(value_name = "PORTS", help = "22,80,1000-1010")]
        ports: String,

        #[arg(value_name = "HOST", help = "192.168.2.65/google.com")]
        target: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Port { ports, target } => {
    
            let parsed_ports = parse_list(&ports).expect("Failed to parse list");
            let (tx, mut rx) = mpsc::channel::<u16>(100);
            for chunk in parsed_ports.chunks(50) {
                let chunk_vec = chunk.to_vec();
                let tx_clone = tx.clone();
                let addr_clone = target.clone();

                tokio::spawn(async move {
                    for port in chunk_vec {
                        port::scan_port(tx_clone.clone(), &addr_clone, port).await;
                    }
                });
            }
            drop(tx);

            let mut open_ports = Vec::new();
            while let Some(port) = rx.recv().await {
                open_ports.push(port);
            }
            open_ports.sort_unstable();
            for port in open_ports {
                println!("Port {} is open", port);
            }
        }
    }
}
