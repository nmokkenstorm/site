use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Header() -> Html {
    html! {
      <header>
        <div>
          <h1>
            {"Hello World!"}
          </h1>
          <nav style="display: flex; gap: 5px">
            <Link<Route> to={Route::Home}>{ "home" }</Link<Route>>
            <Link<Route> to={Route::Blog}>{ "blog" }</Link<Route>>
          </nav>
        </div>
        <p>
          {"This is meant as a personal website and excuse to mess with some non-standard technology"}
        </p>
      </header>
    }
}
