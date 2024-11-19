use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Variable{
    pub key: String,
    pub value: String,
}

