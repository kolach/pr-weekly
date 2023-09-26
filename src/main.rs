mod email;
mod gh;

use chrono::{Duration, Utc};
// use kv_log_macro as log;
use clap::Parser;
use std::fs::File;
use std::io::Write;
use tracing::*;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::{fmt, prelude::*};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Github repository to watch
    #[arg(short, long, env)]
    repo: String,

    // Github API endpoint to send requests
    #[arg(long, env)]
    github_api_endpoint: String,

    // Github API token to authorize requests
    #[arg(long, env)]
    github_api_token: String,

    /// Email address to send report
    #[arg(short, long)]
    send_to: String,
}

impl Args {
    // Compose GH GQL query to get pull requests
    fn query(&self) -> String {
        // Get the current UTC date and time
        let current_date_time = Utc::now();
        // Subtract 7 days from the current date
        let seven_days_ago = current_date_time - Duration::days(7);
        // Format the date as a string in 'YYYY-MM-DD' format
        let formatted_date = seven_days_ago.format("%Y-%m-%d").to_string();

        format!(
            "repo:{} is:pr is:open created:>{}",
            self.repo, formatted_date
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::registry()
        .with(fmt::layer().json())
        .with(EnvFilter::from_default_env())
        .init();

    dotenv::dotenv().ok();

    let args = Args::parse();

    let query = args.query();
    debug!(query = query);

    // Building GH client
    let gh_client = gh::Client::builder()
        .endpoint_url(args.github_api_endpoint)
        .user_agent("pr_weekly/0.0.1")
        .token(args.github_api_token)
        .build()?;

    // Getting pull requests
    let pull_requests = gh_client
        .pull_requests(gh::pull_requests_view::Variables { query })
        .await?;

    // info!(data = serde_json::to_string(&pull_requests).unwrap());

    if let Some(pull_requests) = pull_requests {
        info!(pr_count = pull_requests.len());

        let content = email::render(&pull_requests)?;
        // Create a file
        let mut html_file = File::create("index.html").expect("creation failed");
        // Write contents to the file
        html_file.write(content.as_bytes()).expect("write failed");
    } else {
        warn!("No PR found for last 7 days");
    }

    Ok(())
}
