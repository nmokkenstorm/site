use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ListItem {
    pub title: String,
}

#[derive(Properties, PartialEq)]
pub struct ListProps {
    pub title: String,
    pub items: Vec<ListItem>,
}

#[function_component]
pub fn List(ListProps { title, items }: &ListProps) -> Html {
    let data: Html = items
        .iter()
        .map(|item: &ListItem| -> Html {
            html! {
                <li key={&*item.title}>
                    {format!("{}", item.title)}
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
