use crate::domain::BlogPost;
use contentful::{ContentfulClient, QueryBuilder};
use serde::ser::StdError;

pub async fn get_posts() -> Result<Vec<BlogPost>, Box<dyn StdError>> {
    let space = std::env!("CONTENTFUL_SPACE_ID");
    let token = std::env!("CONTENTFUL_DELIVERY_TOKEN");

    let contentful_client = ContentfulClient::new(&token, &space);

    let builder = QueryBuilder::new().content_type_is("blogPost");

    let posts = contentful_client
        .get_entries::<BlogPost>(Some(builder))
        .await;

    posts
}
