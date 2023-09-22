use graphql_client::GraphQLQuery;

#[allow(clippy::upper_case_acronyms)]
type URI = String;

type DateTime = String;

type GitTimestamp = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gh/gql/schema.graphql",
    query_path = "src/gh/gql/pr_query.graphql",
    response_derives = "Debug, Serialize"
)]
pub struct PullRequestsView;
