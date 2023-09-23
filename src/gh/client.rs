use graphql_client::reqwest::post_graphql;
use std::result::Result;

use super::api::{pull_requests_view, PullRequestsView};
use super::error::GithubError;
use super::pull_requests_view::{PullRequestsViewSearchEdgesNode, PullRequestsViewSearchEdgesNodeOnPullRequest};

#[derive(Clone)]
pub struct Client {
    endpoint_url: reqwest::Url,
    client: reqwest::Client,
}

impl Client {
    pub fn builder() -> GithubBuilder {
        GithubBuilder::new()
    }

    pub async fn pull_requests<V>(
        &self,
        variables: V,
    ) -> Result<Option<Vec<PullRequestsViewSearchEdgesNodeOnPullRequest>>, GithubError>
    where
        V: Into<pull_requests_view::Variables>,
    {
        let endpoint_url = self.endpoint_url.clone();
        let variables = variables.into();
        let response_body =
            post_graphql::<PullRequestsView, _>(&self.client, endpoint_url, variables).await?;

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

        Ok(prs)
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
