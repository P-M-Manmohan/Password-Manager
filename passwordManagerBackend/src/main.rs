mod cli;
mod salt;

use clap::Parser;
use cli::{Commands, Cli};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Add {service, username, password} => {
            println!("Adding the credentials to the service: {}",service);
            println!("Adding the Username: {}",username);
            println!("Adding the password: {}",password);
        }

         Commands::Get { service } => {
            println!("fetching the password for: {}",service)
        }

        Commands::Delete { service } => {
            println!("Deleting the password for: {}",service)
        }
    }

}
