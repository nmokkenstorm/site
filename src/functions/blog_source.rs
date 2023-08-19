use crate::domain::BlogPost;

pub async fn get_posts() -> Result<Vec<BlogPost>, Box<dyn StdError>> {
    let space = env!("CONTENTFUL_SPACE_ID");
    let token = env!("CONTENTFUL_DELIVERY_TOKEN");

    let contentful_client = ContentfulClient::new(&token, &space);

    let builder = QueryBuilder::new().content_type_is("blogPost");

    let posts = contentful_client
        .get_entries::<BlogPost>(Some(builder))
        .await;

    posts
}
