use graphql_client::GraphQLQuery;
use serde::Serialize;

// type GitTimestamp = String;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gh/gql/schema.graphql",
    query_path = "src/gh/gql/pr_query.graphql",
    response_derives = "Debug, Serialize"
)]
pub struct PullRequestsView;

#[derive(Default, Debug, Serialize)]
pub struct Summary {
    pub open: i32,
    pub closed: i32,
    pub merged: i32,
    pub draft: i32,
}
