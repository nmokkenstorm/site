use yew::prelude::*;

use crate::domain::Project;
use crate::functions::get_projects;
use crate::components::{ProjectList, WordList};

#[function_component]
pub fn App() -> Html {

    let skills : Vec<String> = vec!();
    let projects : Vec<Project> = get_projects();

    html! {
        <div>
            <header>  
                <h1>{"Hello World!"}</h1>
                <p>
                    {"This is meant as a personal website and excuse to mess with some non-standard technology"}
                </p>
            </header>
            <main>
                <WordList items={skills} />
                <ProjectList title="Projects" items={projects} />
            </main>
            <footer>
                <p>
                    {"Built with ♥ in "}
                    <a href="https://www.rust-lang.org">{"Rust"}</a>
                    {" and "}
                    <a href="https://www.yew.rs">{"Yew"}</a>
                    {", source available on "}
                    <a href="https://www.github.com/nmokkenstorm/site">{"GitHub"}</a>
                </p>
            </footer>
        </div>
    }
}
