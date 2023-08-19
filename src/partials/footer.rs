use yew::prelude::*;

#[function_component]
pub fn Footer() -> Html {
    html! {
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
    }
}
