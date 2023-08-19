use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize)]
pub struct Project {
    pub name: String,
    pub roles: Vec<String>,
    pub skills: Vec<String>,
    pub parent: Option<String>,
}

#[derive(PartialEq, Deserialize, Serialize)]
pub struct BlogPost {
    pub title: String,
}
