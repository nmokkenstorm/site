use crate::domain::Project;

pub fn get_projects() -> Vec<Project> {
    serde_json::from_slice(include_bytes!("projects.json")).unwrap()
}
