use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::{List, ListItem};
use crate::domain::BlogPost;
use crate::functions::get_posts;

impl From<&BlogPost> for ListItem {
    fn from(post: &BlogPost) -> Self {
        ListItem {
            title: post.title.clone(),
        }
    }
}

#[function_component]
pub fn Blog() -> Html {
    let state = use_state(|| vec![]);
    let loading = use_state(|| true);

    {
        let state = state.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    match get_posts().await {
                        Ok(posts) => {
                            state.set(
                                posts
                                    .iter()
                                    .map(|post| -> ListItem { ListItem::from(post) })
                                    .collect::<Vec<ListItem>>(),
                            );
                        }
                        Err(_) => {
                            state.set(vec![]);
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
      <List title="Blog Posts" items={(*state).clone()} />
    }
}
