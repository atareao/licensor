mod configuration;
mod template;
mod variable;
mod tools;
mod cli;

use inquire::Select;
use simple_logger::init_with_env;
use log::{info, debug, error};
use cli::{Cli, Commands};
use clap::Parser;
use configuration::Configuration;

#[tokio::main]
async fn main() {
    info!("Starting...");
    init_with_env().expect("Cant init logger");
    let configuration = Configuration::new().await;
    debug!("{:?}", configuration);
    let cli = Cli::parse();
    match &cli.command {
        Commands::Update => {
            let url = configuration.url;
            debug!("{:?}", &url);
            let filename = "/tmp/output.zip";
            let _ = tools::fetch_url(&url, filename).await;

        },
        Commands::Select => {
            let templates: Vec<String> = configuration
                .templates
                .iter()
                .map(|s| s.name.clone())
                .collect();
            debug!("{:?}", &templates);
            let ans = Select::new("Select license?", templates)
                .prompt();
            debug!("{:?}", &ans);
        },
    }
}
