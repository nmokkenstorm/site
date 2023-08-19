use yew::prelude::*;

use crate::components::{List, ListItem, WordList};
use crate::domain::Project;
use crate::functions::get_projects;

impl From<&Project> for ListItem {
    fn from(project: &Project) -> Self {
        ListItem {
            title: project.name.clone(),
        }
    }
}

#[function_component]
pub fn Home() -> Html {
    let skills: Vec<String> = vec![];
    let projects: Vec<ListItem> = get_projects()
        .iter()
        .map(|project| -> ListItem { ListItem::from(project) })
        .collect::<Vec<ListItem>>();

    html! {
        <>
            <WordList items={skills} />
            <List title="Projects" items={projects} />
        </>

    }
}
