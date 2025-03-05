use clap::builder::TypedValueParser;
use rand::{distributions::Alphanumeric, Rng};
use dirs;
use std::{env, fs, path::PathBuf, process};


// check for config file to store salt in for persistence and create said file if it does not
// exist
fn get_config_path() -> PathBuf {
    
    let home_dir = dirs::home_dir().expect("failed to find home dir");
    let config_path = home_dir.join(".password_config_file");

    if !config_path.exists() {
        if let Err(err) = fs::write(&config_path, "") {
            eprintln!("Error creating config file: {}",err);
        }
    }

    config_path
}

//generate random Alphanumeric salt of size 32 characters
fn generate_salt() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

//save the salt to the config file
fn save_salt_to_file(salt: &str) {
    let config_path = get_config_path();
    if let Err(err) = fs::write(config_path,salt){
        eprintln!("Error writing salt to config file, {}", err);
        process::exit(1);
    }
}

//load the salt from the config file
fn load_salt_from_file() -> Option<String> {
    let config_path = get_config_path();
    fs::read_to_string(config_path).ok()
}

fn check_and_set_salt_as_env() {
    let salt: String;
    dotenvy::dotenv().ok();

}

