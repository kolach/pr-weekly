mod email;
mod gh;

use clap::Parser;
use lettre::message::Mailbox;
use tracing::*;
use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*};

/// Command args
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
    #[arg(short, long, env)]
    send_to: Mailbox,

    /// Email is sent from
    #[arg(short, long, env)]
    from: Mailbox,

    /// SMTP host
    #[arg(long, env)]
    smtp_host: String,

    /// SMTP port
    #[arg(long, env, default_value_t = 2525)]
    smtp_port: u16,

    /// SMTP credentials user
    #[arg(long, env)]
    smtp_user: String,

    /// SMTP credencials pass
    #[arg(long, env)]
    smtp_pass: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::registry()
        .with(fmt::layer().json())
        .with(EnvFilter::from_default_env())
        .init();

    dotenv::dotenv().ok();

    let args = Args::parse();

    trace!(
        repo = args.repo,
        to = format!("{:?}", args.send_to),
        from = format!("{:?}", args.from)
    );

    // Building GH client
    let gh_client = gh::Client::builder()
        .endpoint_url(args.github_api_endpoint)
        .user_agent("pr_weekly/0.0.1")
        .token(args.github_api_token)
        .build()?;

    // Getting pull requests
    let summary = gh_client.pull_requests_summary(&args.repo).await?;

    if let Some(summary) = summary {
        info!(summary = serde_json::to_string(&summary).unwrap());

        let subject = format!("Summary of {} PRs in last 7 days", args.repo);
        let content = email::render(&subject, &summary)?;

        trace!(email_subject = subject, email_content = content);

        let sender = email::Sender::new(
            args.smtp_host,
            args.smtp_port,
            args.smtp_user,
            args.smtp_pass,
        );

        sender
            .send(&subject, args.from, args.send_to, content)
            .await?;
    }

    Ok(())
}
