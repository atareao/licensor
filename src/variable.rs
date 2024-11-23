use serde::{Serialize, Deserialize};
use yaml_rust2::{Yaml, yaml::Hash};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Variable{
    pub key: String,
    pub value: String,
}

impl Variable {
    pub fn to_yml(&self) -> Yaml{
        let mut hash = Hash::new();
        hash.insert(Yaml::String("key".to_string()), Yaml::String(self.key.clone()));
        hash.insert(Yaml::String("value".to_string()), Yaml::String(self.value.clone()));
        Yaml::Hash(hash)
    }
    pub fn from_yml(yml: &Yaml) -> Self{
        let key = yml["key"].as_str().unwrap().to_string();
        let value = yml["value"].as_str().unwrap().to_string();
        Self{
            key,
            value,
        }
    }
}
