mod configuration;
mod template;
mod variable;
mod tools;

use simple_logger::init_with_env;
use log::{info, debug, error};
use configuration::Configuration;

#[tokio::main]
async fn main() {
    info!("Starting...");
    init_with_env().expect("Cant init logger");
    let configuration = Configuration::new().await;
    debug!("{:?}", configuration);
    let templates: Vec<String> = configuration
        .templates
        .iter()
        .map(|s| s.name.clone())
        .collect();
    debug!("{:?}", &templates);
    let ans: Result<&str, InquireError> = Select::new("Select license?", templates)
        .prompt();
}
