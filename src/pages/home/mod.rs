use crate::pages::home::query::{Film, FilmsConnection, FilmsQuery};
use cynic::GraphQlResponse;
use cynic::QueryBuilder;
use leptos::*;

mod query;

pub async fn films_query(_wow: i32) -> Vec<Option<Film>> {
    let operation = FilmsQuery::build(());
    let response = reqwest::Client::new()
        .get("https://swapi-graphql.netlify.app/.netlify/functions/index")
        .json(&operation)
        .send()
        .await
        .unwrap();
    response
        .json::<GraphQlResponse<FilmsQuery>>()
        .await
        .unwrap()
        .data
        .unwrap()
        .all_films
        .unwrap()
        .films
        .unwrap()
}

#[component]
pub fn HomePage() -> impl IntoView {
    let (ok, set_ok) = create_signal(0);
    let films_resources = create_resource(move || ok.clone().get(), |o| films_query(o));
    view! {
      <>
      <h1> Home Page </h1>
      // <Suspense
      //   fallback= move || view! {<p> loading ...</p>}
      // >
      // {move||{
      //   films_resources.get().map(|films| view! {
      //     <For
      //       each=move|| films.clone().unwrap().all_films.unwrap().films.unwrap().clone()
      //       key= |o| o.clone().unwrap().title
      //       children=|fi| {
      //       view! {<p>{fi.clone().unwrap().title}</p>}
      //     }
      //     />
      //   })
      // }}
      // </Suspense>

      </>
    }
}
