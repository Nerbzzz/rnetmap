use std::net::{IpAddr, SocketAddr};
use tokio::sync::mpsc;
use clap::{Parser, Subcommand};

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

        #[arg(value_name = "HOST", help = "10.0.0.1-10.0.0.4,10.0.0.5,10.0.0.0/28")]
        target: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Port { ports, target } => {

            let parsed_ports: Vec<u16> = util::parse_ports(&ports).expect("Failed to parse list");
            let parsed_ips: Vec<IpAddr> = util::parse_ips(&target).await.expect("Failed to parse hosts");

            let (tx, mut rx) = mpsc::channel::<SocketAddr>(100);
            for ip in parsed_ips {
                for &port in &parsed_ports {
                    let tx_clone = tx.clone();
                    let addr = SocketAddr::new(ip, port);
                    tokio::spawn(async move {
                        port::scan_port(tx_clone, addr).await;
                    });
                }
            }
            drop(tx);

            while let Some(open_addr) = rx.recv().await {
                println!("Port {} is open on {}", open_addr.port(), open_addr.ip());
            }


        }
    }
}
