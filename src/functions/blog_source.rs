use crate::domain::BlogPost;

pub fn get_posts() -> Vec<BlogPost> {
    let mut data: Vec<BlogPost> = Vec::new();

    data.push(BlogPost {
        title: "My first post".to_owned(),
    });

    data.push(BlogPost {
        title: "My second post".to_owned(),
    });

    data
}
