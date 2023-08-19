use yew::prelude::*;

use crate::components::{ProjectList, WordList};
use crate::domain::Project;
use crate::functions::get_projects;

#[function_component]
pub fn Home() -> Html {
    let skills: Vec<String> = vec![];
    let projects: Vec<Project> = get_projects();

    html! {
        <>
            <WordList items={skills} />
            <ProjectList title="Projects" items={projects} />
        </>

    }
}
