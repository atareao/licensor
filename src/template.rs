use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Template{
    pub name: String,
    pub filename: String,
}
