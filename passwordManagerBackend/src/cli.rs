use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dashline")]
#[command(about = "a simple password manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    //Add password
    Add {
        #[arg(short, long)]
        service: String,

        #[arg(short, long)]
        username: String,

        #[arg(short, long)]
        password: String,
    },
    //Retreive Password
    Get {
        #[arg(short, long)]
        service: String,
    },

    //Remove password
    Delete {
        #[arg(short, long)]
        service: String,
    },
}
