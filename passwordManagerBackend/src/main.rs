mod cli;
mod encryption;
mod salt;

use clap::Parser;
use cli::{Commands, Cli};
use salt::check_and_set_salt_as_env;
use encryption::{encrypt_data,decrypt_data,derive_key};
use rpassword::read_password;

fn main() {

    let salt = check_and_set_salt_as_env();
    println!("enter master password: ");
    let master = read_password().expect("failed to read password");
    let key = derive_key(&master, &salt);
    let args = Cli::parse();
    let nonce_user : String;
    let nonce_pass : String;
    let nonce_service : String;
    let username_en: String;
    let password_en: String;
    let service_en: String;
    match args.command {
        Commands::Add {service,username,password} => {
            println!("{:?}",key);
            println!("Adding the credentials to the service: {}",service);
            println!("Adding the Username: {}",username);
            println!("Adding the password: {}",password);
            (nonce_user,username_en)=encrypt_data(&key, &username);
            (nonce_pass,password_en)=encrypt_data(&key, &password);
            (nonce_service,service_en)=encrypt_data(&key, &service);
            println!("Adding the credentials to the service:{} {}",nonce_service,service_en);
            println!("Adding the Username:{} {}",nonce_user,username_en);
            println!("Adding the password:{} {}",nonce_pass,password_en);
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
