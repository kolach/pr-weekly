mod email;
mod gh;

// use kv_log_macro as log;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt().json().init();

    let default_github_endpoint = String::from("https://api.github.com/graphql");
    let github_api_token =
        std::env::var("GITHUB_API_TOKEN").expect("Missing GITHUB_API_TOKEN env var");

    let gh_client = gh::Client::builder()
        .endpoint_url(default_github_endpoint)
        .user_agent("pr_weekly/0.0.1")
        .token(github_api_token)
        .build()?;

    let query = "repo:rails/rails is:pr is:open created:>2023-09-20".to_string();

    let pull_requests = gh_client
        .pull_requests(gh::pull_requests_view::Variables { query })
        .await?;

    info!(data = serde_json::to_string(&pull_requests).unwrap());

    if let Some(pull_requests) = pull_requests {
        let content = email::render(&pull_requests)?;
        info!(content = content);
    }

    Ok(())
}
