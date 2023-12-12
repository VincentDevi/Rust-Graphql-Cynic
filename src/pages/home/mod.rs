use cynic::QueryBuilder;
use leptos::*;

mod query;

#[component]
pub fn HomePage() -> impl IntoView {
    let (ok, set_ok) = create_signal(0);
    view! {
      <>
      <h1> Home Page </h1>

      </>
    }
}
