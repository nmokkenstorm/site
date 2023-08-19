use yew::prelude::*;

use crate::domain::Project;

#[derive(Properties, PartialEq)]
pub struct ProjectListProps {
    pub title: String,
    pub items: Vec<Project>,
}

#[function_component]
pub fn ProjectList(ProjectListProps { title, items }: &ProjectListProps) -> Html {
    let data: Html = items
        .iter()
        .map(|project: &Project| -> Html {
            html! {
                <li key={&*project.name}>
                    {format!("{}", project.name)}
                </li>
            }
        })
        .collect::<Html>();

    html! {
        <div>
            <h2>{title}</h2>
            <ul>{data}</ul>
        </div>
    }
}
