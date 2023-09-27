use chrono::{Duration, Utc};
use graphql_client::{GraphQLQuery, Response};
use std::result::Result;

use super::api::{pull_requests_view, PullRequestsView};
use super::error::GithubError;
use super::pull_requests_view::{PullRequestState, PullRequestsViewSearchEdgesNode};
use super::Summary;

#[derive(Clone)]
pub struct Client {
    endpoint_url: reqwest::Url,
    client: reqwest::Client,
}

// Compose GH GQL query to get pull requests
fn week_query(repo: &str) -> String {
    // Get the current UTC date and time
    let current_date_time = Utc::now();
    // Subtract 7 days from the current date
    let seven_days_ago = current_date_time - Duration::days(7);
    // Format the date as a string in 'YYYY-MM-DD' format
    let formatted_date = seven_days_ago.format("%Y-%m-%d").to_string();

    format!("repo:{} is:pr created:>{}", repo, formatted_date)
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

    // Makes API requests and transforms response into PR summary
    pub async fn pull_requests_summary(&self, repo: &str) -> Result<Option<Summary>, GithubError> {
        let query = week_query(repo);
        let variables = pull_requests_view::Variables { query };
        let response_body = self.post_graphql(variables).await?;

        // let mut summary = Summary::default();

        let summary = response_body
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
                    .fold(Summary::default(), |mut sum, pr| {
                        if pr.is_draft {
                            sum.draft += 1;
                        }
                        match pr.state {
                            PullRequestState::CLOSED => sum.closed += 1,
                            PullRequestState::MERGED => sum.merged += 1,
                            PullRequestState::OPEN => sum.open += 1,
                            _ => (),
                        }
                        sum
                    })
            });

        Ok(summary)
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
