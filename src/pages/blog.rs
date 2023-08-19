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
    let posts = get_posts()
        .iter()
        .map(|post| -> ListItem { ListItem::from(post) })
        .collect::<Vec<ListItem>>();

    html! {
      <List title="Blog Posts" items={posts} />
    }
}
