use yew::prelude::*;

use crate::pages::Home;
use crate::partials::{Footer, Header};

#[function_component]
pub fn App() -> Html {
    html! {
      <div>
        <main>
            <Header/>
            <Home />
        </main>
        <Footer/>
      </div>
    }
}
