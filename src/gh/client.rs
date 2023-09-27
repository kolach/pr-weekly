use graphql_client::{GraphQLQuery, Response};
use std::result::Result;

use super::api::{pull_requests_view, PullRequestsView};
use super::error::GithubError;
use super::pull_requests_view::{
    PullRequestsViewSearchEdgesNode, PullRequestsViewSearchEdgesNodeOnPullRequest,
};

#[derive(Clone)]
pub struct Client {
    endpoint_url: reqwest::Url,
    client: reqwest::Client,
}

impl Client {
    pub fn builder() -> GithubBuilder {
        GithubBuilder::new()
    }

    // Here we check the HTTP response and convert it to our grapth ql response body.
    // This is a place to handle the error responses from GitHub API service
    async fn post_graphql<V>(
        &self,
        variables: V,
    ) -> Result<Response<<PullRequestsView as GraphQLQuery>::ResponseData>, GithubError>
    where
        V: Into<pull_requests_view::Variables>,
    {
        let endpoint_url = self.endpoint_url.clone();
        let body = PullRequestsView::build_query(variables.into());
        let reqwest_response = self.client.post(endpoint_url).json(&body).send().await?;

        match reqwest_response.status() {
            reqwest::StatusCode::UNAUTHORIZED => Err(GithubError::Unauthorized),
            _ => {
                let response_body: Response<_> = reqwest_response.json().await?;
                if let Some(errors) = response_body.errors {
                    // This check is for GH response errors.
                    // For example if number of records to fetch exceeds 100,
                    // GH will respond with HTTP status 200 OK but there will
                    // be a error message.
                    return Err(GithubError::QueryError { errors });
                }
                Ok(response_body)
            }
        }
    }

    // Makes API requests and transforms response into a list of PR-like structures
    pub async fn pull_requests<V>(
        &self,
        variables: V,
    ) -> Result<Vec<PullRequestsViewSearchEdgesNodeOnPullRequest>, GithubError>
    where
        V: Into<pull_requests_view::Variables>,
    {
        let response_body = self.post_graphql(variables).await?;

        let prs = response_body
            .data
            .map(|data| data.search)
            .and_then(|search| search.edges)
            .map(|edges| {
                edges
                    .into_iter()
                    .flatten()
                    .filter_map(|edge| edge.node)
                    .filter_map(|node| {
                        if let PullRequestsViewSearchEdgesNode::PullRequest(pr) = node {
                            return Some(pr);
                        }
                        None
                    })
                    .collect::<Vec<_>>()
            });

        Ok(prs.unwrap_or(vec![]))
    }
}

#[derive(Default, Clone)]
pub struct GithubBuilder {
    endpoint_url: String,
    user_agent: String,
    token: String,
}

impl GithubBuilder {
    pub fn new() -> GithubBuilder {
        GithubBuilder {
            endpoint_url: String::from("https://api.github.com/graphql"),
            user_agent: String::from("graphql-rust/0.10.0"),
            token: String::from(""),
        }
    }

    pub fn endpoint_url<S: Into<String>>(mut self, endpoint_url: S) -> GithubBuilder {
        self.endpoint_url = endpoint_url.into();
        self
    }

    pub fn user_agent<S: Into<String>>(mut self, user_agent: S) -> GithubBuilder {
        self.user_agent = user_agent.into();
        self
    }

    pub fn token<S: Into<String>>(mut self, token: S) -> GithubBuilder {
        self.token = token.into();
        self
    }

    pub fn build(self) -> Result<Client, GithubError> {
        let client = reqwest::Client::builder()
            .user_agent(self.user_agent)
            .default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", self.token))
                        .unwrap(),
                ))
                .collect(),
            )
            .build()?;

        Ok(Client {
            endpoint_url: reqwest::Url::parse(&self.endpoint_url)?,
            client,
        })
    }
}
