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
use home::home_dir;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new().await;
    debug!("{:?}", configuration);
    let cli = Cli::parse();
    if cli.debug {
        init_with_env().expect("Cant init logger");
        info!("Starting...");
    }
    match &cli.command {
        Commands::Update => {
            let url = configuration.url;
            debug!("{:?}", &url);
            let zipfile = "/tmp/output.zip";
            let mut config_dir = home_dir().unwrap();
            config_dir.push(".config");
            config_dir.push("licensor");
            config_dir.push("licenses");
            let y = tokio::fs::remove_dir_all(config_dir.clone()).await;
            debug!("{:?}", y);
            config_dir.pop();
            let _ = tools::fetch_url(&url, zipfile).await;
            let _ = tools::unzip(zipfile, &config_dir).await;
            let _ = tokio::fs::remove_file(zipfile).await;
            let mut original = config_dir.clone();
            original.push("licensor-templates-main");
            let mut new = config_dir.clone();
            new.push("licenses");
            let _ = tokio::fs::rename(original, new).await;
            let _ = Configuration::update_licenses().await;
        },
        Commands::Select => {
            let mut templates: Vec<String> = configuration
                .templates
                .iter()
                .map(|s| s.name.clone())
                .collect();
            templates.sort();
            debug!("{:?}", &templates);
            match Select::new("Select license?", templates).prompt(){
                Ok(selected) => {
                    debug!("{:?}", &selected);
                    for template in configuration.templates{
                        if template.name == selected{
                            let mut license_file = home_dir().unwrap();
                            license_file.push(".config");
                            license_file.push("licensor");
                            license_file.push("licenses");
                            license_file.push(template.filename);
                            let license_content = tokio::fs::read_to_string(&license_file).await.unwrap();
                            debug!("{:?}", &license_file);
                            let license = tools::reder_template(&configuration.variables, &license_file, &license_content).unwrap();
                            let _ = tokio::fs::write("LICENSE", license).await;
                        }
                    }
                },
                Err(e) => {
                    error!("{:?}", e);
                }
            }
        }
    }
}
