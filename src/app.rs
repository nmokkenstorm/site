use yew::prelude::*;

use crate::components::{ProjectList, WordList};
use crate::domain::Project;
use crate::functions::get_projects;
use crate::partials::{Footer, Header};

#[function_component]
pub fn App() -> Html {
    let skills: Vec<String> = vec![];
    let projects: Vec<Project> = get_projects();

    html! {
      <div>
        <main>
          <Header/>
          <WordList items={skills} />
          <ProjectList title="Projects" items={projects} />
        </main>
        <Footer/>
      </div>
    }
}
