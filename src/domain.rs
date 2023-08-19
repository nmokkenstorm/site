use serde::Deserialize;

#[derive(PartialEq, Deserialize)]
pub struct Project {
    pub name: String,
    pub roles: Vec<String>,
    pub skills: Vec<String>,
    pub parent: Option<String>,
}
