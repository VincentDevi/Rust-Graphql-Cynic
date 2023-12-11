use serde::Serialize;

mod schema {
    cynic::use_schema!("schema.graphql");
}

#[derive(cynic::QueryFragment, Debug, Serialize, Clone)]
#[cynic(graphql_type = "Root")]
pub struct FilmsQuery {
    pub all_films: Option<FilmsConnection>,
}

#[derive(cynic::QueryFragment, Debug, Serialize, Clone)]
pub struct FilmsConnection {
    pub films: Option<Vec<Option<Film>>>,
}

#[derive(cynic::QueryFragment, Debug, Serialize, Clone)]
pub struct Film {
    pub title: Option<String>,
}
