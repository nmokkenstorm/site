use yew::prelude::*;

#[function_component]
pub fn Header() -> Html {
    html! {
      <header>
        <h1>
          {"Hello World!"}
        </h1>
        <p>
          {"This is meant as a personal website and excuse to mess with some non-standard technology"}
        </p>
      </header>
    }
}
