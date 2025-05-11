use clap::{Parser, Subcommand};

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
        Commands::Port { ports: _, target: _ } => {

        }
    }
}
