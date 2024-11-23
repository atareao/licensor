use serde::{Serialize, Deserialize};
use yaml_rust2::{Yaml, yaml::Hash};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template{
    pub name: String,
    pub filename: String,
}

impl Template {
    pub fn to_yml(&self) -> Yaml{
        let mut hash = Hash::new();
        hash.insert(Yaml::String("name".to_string()), Yaml::String(self.name.clone()));
        hash.insert(Yaml::String("filename".to_string()), Yaml::String(self.filename.clone()));
        Yaml::Hash(hash)
    }
    pub fn from_yml(yml: &Yaml) -> Self{
        let name = yml["name"].as_str().unwrap().to_string();
        let filename = yml["filename"].as_str().unwrap().to_string();
        Self{
            name,
            filename,
        }
    }
}

impl From<Template> for Yaml {
    fn from(template: Template) -> Self {
        let mut hash = Hash::new();
        hash.insert(Yaml::String("name".to_string()), Yaml::String(template.name.clone()));
        hash.insert(Yaml::String("filename".to_string()), Yaml::String(template.filename.clone()));
        Yaml::Hash(hash)
    }
}
impl From<Yaml> for Template {
    fn from(yaml: Yaml) -> Self {
        yaml.as_hash().map(|hash| {
            let name = hash.get(&Yaml::String("name".to_string())).unwrap().as_str().unwrap().to_string();
            let filename = hash.get(&Yaml::String("filename".to_string())).unwrap().as_str().unwrap().to_string();
            Self{
            name,
            filename,
            }
        }).unwrap()
    }
}
