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
use serde::{Serialize, Deserialize};
use home::home_dir;
use yaml_rust2::{Yaml, YamlLoader, YamlEmitter, yaml::Hash};
use tokio::fs;
use super::template::Template;
use super::variable::Variable;

static DEFAULT_CONFIG: &str = include_str!("./licensor.yml");

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    #[serde(default = "get_default_url")]
    pub url: String,
    #[serde(default = "get_default_templates")]
    pub templates: Vec<Template>,
    #[serde(default = "get_default_variables")]
    pub variables: Vec<Variable>,
}

impl Configuration {
    pub fn to_yml(&self) -> Yaml {
        let mut hash = Hash::new();
        hash.insert(Yaml::String("url".to_string()), Yaml::String(self.url.clone()));
        let templates = self.templates
            .iter()
            .map(|template| template.to_yml())
            .collect::<Vec<Yaml>>();
        hash.insert(Yaml::String("templates".to_string()), Yaml::Array(templates));
        let variables = self.variables
            .iter()
            .map(|variable| variable.to_yml())
            .collect::<Vec<Yaml>>();
        hash.insert(Yaml::String("variables".to_string()), Yaml::Array(variables));
        Yaml::Hash(hash)
    }

    pub fn from_yml(yml: Yaml) -> Self{
        let url = yml["url"].as_str().unwrap_or(&get_default_url()).to_string();
        let templates = yml["templates"]
            .as_vec()
            .unwrap()
            .iter()
            .map(Template::from_yml)
            .collect::<Vec<Template>>();
        let variables = yml["variables"]
            .as_vec()
            .unwrap()
            .iter()
            .map(Variable::from_yml)
            .collect::<Vec<Variable>>();
        Self {
            url,
            templates,
            variables,
        }
    }

    pub async fn update_licenses() -> Result<(), Error>{
        let licenses_content = read_licenses().await?;
        let items = YamlLoader::load_from_str(&licenses_content)
            .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
        debug!("{:?}", items[0]);
        let templates = items[0]
            .as_vec()
            .unwrap()
            .iter()
            .map(Template::from_yml)
            .collect::<Vec<Template>>();
        let mut config = Self::new().await;
        config.templates = templates;
        config.save().await?;
        Ok(())
    }

    pub async fn save(&self) -> Result<(), Error> {
        let mut output = String::new();
        let mut emitter = YamlEmitter::new(&mut output);
        emitter.multiline_strings(true);
        emitter.dump(&self.to_yml()).unwrap();
        debug!("output: {:?}", self.to_yml());
        let mut config_file = home_dir().unwrap();
        config_file.push(".config");
        config_file.push("licensor");
        config_file.push("licensor.yml");
        debug!("output: {:?}", output);
        tokio::fs::write(config_file, output).await?;
        
        Ok(())
    }

    pub async fn new()->Self{
        let config_content = match read_file().await{
            Ok(content) => content,
            Err(_) => {
                create_default().await.unwrap()
            }
        };
        let items = YamlLoader::load_from_str(&config_content).unwrap();
        Self::from_yml(items[0].clone())
    }
}

fn get_default_url() -> String{
    "https://github.com/atareao/licensor-templates/archive/main.zip".to_string()
}

fn get_default_templates() -> Vec<Template>{
    Vec::new()
}

fn get_default_variables() -> Vec<Variable>{
    Vec::new()
}

async fn read_licenses() -> Result<String, Error> {
    info!("read_file");
    let mut config_dir = home_dir().unwrap();
    config_dir.push(".config");
    config_dir.push("licensor");
    config_dir.push("licenses");
    config_dir.push("licenses.yml");
    debug!("config_dir: {:?}", config_dir);
    tokio::fs::read_to_string(config_dir.to_str().unwrap()).await

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
