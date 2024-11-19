// Copyright (c) 2024 Lorenzo Carbonell <a.k.a. atareao>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use std::io::Error;
use log::{info, debug};
use serde::Deserialize;
use home::home_dir;
use yaml_rust2::YamlLoader;
use tokio::fs;
use super::template::Template;
use super::variable::Variable;

static DEFAULT_CONFIG: &str = include_str!("./licensor.yml");

#[derive(Deserialize, Debug)]
pub struct Configuration {
    #[serde(default = "get_default_url")]
    pub url: String,
    #[serde(default = "get_default_templates")]
    pub templates: Vec<Template>,
    #[serde(default = "get_default_variables")]
    pub variables: Vec<Variable>,
}

impl Configuration {
    pub async fn new()->Self{
        let config_content = match read_file().await{
            Ok(content) => content,
            Err(_) => {
                create_default().await.unwrap()
            }
        };
        let items = YamlLoader::load_from_str(&config_content).unwrap();
        let item = &items[0];
        let url = item["url"].as_str().unwrap_or(&get_default_url()).to_string();
        let templates = item["templates"]
            .as_vec()
            .unwrap()
            .iter()
            .map(|item| Template{
                name: item["name"].as_str().unwrap().to_string(),
                filename: item["filename"].as_str().unwrap().to_string(),
            })
                .collect::<Vec<Template>>()
        ;
        let variables = item["variables"]
            .as_vec()
            .unwrap()
            .iter()
            .map(|item| Variable{
                key: item["key"].as_str().unwrap().to_string(),
                value: item["value"].as_str().unwrap().to_string(),
            })
            .collect::<Vec<Variable>>()
        ;
        Configuration{
            url,
            templates,
            variables,
        }
    }
}

fn get_default_url() -> String{
    "https://github.com/github/licensor-templates".to_string()
}

fn get_default_templates() -> Vec<Template>{
    Vec::new()
}

fn get_default_variables() -> Vec<Variable>{
    Vec::new()
}

async fn read_file()->Result<String, Error>{
    info!("read_file");
    let mut config_dir = home_dir().unwrap();
    config_dir.push(".config");
    config_dir.push("licensor");
    config_dir.push("licensor.yml");
    debug!("config_dir: {:?}", config_dir);
    tokio::fs::read_to_string(config_dir.to_str().unwrap()).await
}

async fn create_default() -> Result<String, Error>{
    info!("created_default");
    let mut config_dir = home_dir().unwrap();
    config_dir.push(".config");
    config_dir.push("licensor");
    fs::create_dir_all(&config_dir).await?;
    config_dir.push("licensor.yml");
    fs::write(config_dir, DEFAULT_CONFIG).await?;
    Ok(DEFAULT_CONFIG.to_string())
}
