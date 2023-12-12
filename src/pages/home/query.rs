use serde::Serialize;

mod schema {
    cynic::use_schema!("schema.graphql");
}

#[derive(cynic::QueryFragment, Debug, Serialize, Clone)]
#[cynic(graphql_type = "Query")]
pub struct MyQuery {
    #[arguments(first: 5)]
    pub orders: Vec<Order>,
}

#[derive(cynic::QueryFragment, Debug, Serialize, Clone)]
pub struct Order {
    pub category: Category,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum Category {
    #[cynic(rename = "parcel")]
    Parcel,
    #[cynic(rename = "estate")]
    Estate,
    #[cynic(rename = "wearable")]
    Wearable,
    #[cynic(rename = "ens")]
    Ens,
}
