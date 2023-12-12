use leptos::*;
use leptos_router::*;
use crate::pages::*;

#[component(transparent)]
pub fn AppRouter()->impl IntoView{
  view! {
    <Router>
      <Routes>
        <Route path="" view=HomePage/>
        <Route path="/films" view=|| view! { <p>Films</p> }/>
        <Route path="/directors" view=|| view! { <p>Directors</p> }/>
      </Routes>
    </Router>
  }
}
