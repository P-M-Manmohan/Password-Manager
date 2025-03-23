mod cli;
mod encryption;
mod salt;
mod database;
mod queries;
mod models;
mod clipboard;

use clap::Parser;
use cli::{Commands, Cli};
use queries::{add_password, get_credentials, delete_credentials};
use salt::check_and_set_salt_as_env;
use encryption::{encrypt_data,decrypt_data,derive_key};
use database::{init_db_pool,DbPool};
use clipboard::copy_to_clipboard;
use rpassword::read_password;



#[tokio::main]
async fn main() {

    let salt = check_and_set_salt_as_env();
    let db_pool: DbPool = init_db_pool().await;
    let _ = sqlx::migrate!("./migrations").run(&db_pool).await;
    println!("enter master password: ");
    let master = read_password().expect("failed to read password");
    let key = derive_key(&master, &salt);
    let args = Cli::parse();
    let nonce_user : String;
    let nonce_pass : String;
    let username_en: String;
    let password_en: String;
    match args.command {
        Commands::Add {service,username,password} => {
            (nonce_user,username_en)=encrypt_data(&key, &username);
            (nonce_pass,password_en)=encrypt_data(&key, &password);
            let _response = add_password(db_pool, &password_en, &username_en, &service, &nonce_user, &nonce_pass).await;
            println!("Password saved for {} ",service);

        }

         Commands::Get { service } => {
            let mut username_str= String::new();
            let mut password_str= String::new();
            let mut nonce_user= String::new();
            let mut nonce_pass= String::new();
            let password_de: String;
            let username_de: String;
            let response = get_credentials(db_pool, &service).await;
            println!("fetching the password for: {}",service);
            match response {
                Ok(res) => {
                    if let Some(username) = res.get("username"){
                        username_str = username.to_string();
                    }  
                    if let Some(password) = res.get("password"){
                        password_str = password.to_string();
                    }
                    if let Some(nonce) = res.get("nonce_pass"){
                        nonce_pass = nonce.to_string();
                    }
                    if let Some(nonce) = res.get("nonce_user"){
                        nonce_user = nonce.to_string();
                    }
                },
                Err(err) => 
                {
                    println!("err {}",err);
                    return;            
                }
            }
            if !username_str.is_empty() && !password_str.is_empty() && !nonce_pass.is_empty() && !nonce_user.is_empty(){
            username_de = match decrypt_data(&key, &nonce_user, &username_str){
                    Ok(username) => username,
                    Err(_) => {
                        println!("Master Password incorrect");
                        return;
                    }
                };
            password_de = match decrypt_data(&key, &nonce_pass, &password_str){
                Ok(password) => password,
                Err(_) => {
                        println!("Master Password incorrect");
                        return;
                    }
            };
                let _= copy_to_clipboard(&password_de);
                println!("username : {}", username_de );
            }


            
        }

        Commands::Delete { service } => {
            println!("Deleting the password for: {}",service);
            let _response = delete_credentials(db_pool, &service).await;
        }
    }

}
