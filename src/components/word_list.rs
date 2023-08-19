use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct WordListProps {
    pub items: Vec<String>,
}

#[function_component]
pub fn WordList(WordListProps { items }: &WordListProps) -> Html {
    let data: Html = items
        .iter()
        .map(|item: &String| -> Html {
            html! {
                <li>
                    {format!("{}", item)}
                </li>
            }
        })
        .collect::<Html>();

    html! {
        <div>
            <ul>{data}</ul>
        </div>
    }
}
