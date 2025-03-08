mod cli;
mod encryption;
mod salt;

use clap::Parser;
use cli::{Commands, Cli};
use salt::check_and_set_salt_as_env;

fn main() {

    check_and_set_salt_as_env();
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
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep to prevent 100% CPU usage
    }

}
