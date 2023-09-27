# PR-Weekly

SailPoint code assignment.

## The Task 

Using the language of your choice, write code that will use the GitHub API to retrieve a summary of all opened, closed, 
and in draft pull requests in the last week for a given repository and send a summary email to a configurable email address. 
Choose any public target GitHub repository you like that has had at least 3 pull requests in the last week. 
Format the content email as you see fit, with the goal to allow the reader to easily digest the events of the past week. 
If sending email is not an option, then please print to console the details of the email you would send 
(From, To, Subject, Body).

## Implementation

The project is implemented using the Rust programming language. I found Rust very convenient to use with the GraphQL API, as it allows writing type-safe code through the extensive usage of its macros.

I can navigate you through the code to explain what I mean if I get a chance.

## Building The Project

Rust is required to compile the project. Use the following link to install it if you want to build the project locally: https://www.rust-lang.org/tools/install

To build the project in debug mode, run:

```
cargo build
```

You'll find the binary in ./target/debug/pr-weekly

To build a release, use:


```
cargo build --release
```

You'll find the binary in ./target/release/pr-weekly

## Running the Project

Use --help outut as a reference for command line arguments:

```
./target/release/pr-weekly --help
Command args

Usage: pr-weekly [OPTIONS] --repo <REPO> --github-api-endpoint <GITHUB_API_ENDPOINT> --github-api-token <GITHUB_API_TOKEN> --send-to <SEND_TO> --from <FROM> --smtp-host <SMTP_HOST> --smtp-user <SMTP_USER> --smtp-pass <SMTP_PASS>

Options:
  -r, --repo <REPO>
          Github repository to watch [env: REPO=rails/kolach]
      --github-api-endpoint <GITHUB_API_ENDPOINT>
          [env: GITHUB_API_ENDPOINT=https://api.github.com/graphql]
      --github-api-token <GITHUB_API_TOKEN>
          [env: GITHUB_API_TOKEN=6da2cb9e1cead68c55eb2319a5faf619fd24033f]
  -s, --send-to <SEND_TO>
          Email address to send report [env: SEND_TO=chikolad@gmail.com]
  -f, --from <FROM>
          Email is sent from [env: FROM=Nikolay Chistyakov <nick@codewire.tech>]
      --smtp-host <SMTP_HOST>
          SMTP host [env: SMTP_HOST=sandbox.smtp.mailtrap.io]
      --smtp-port <SMTP_PORT>
          SMTP port [env: SMTP_PORT=2525] [default: 2525]
      --smtp-user <SMTP_USER>
          SMTP credentials user [env: SMTP_USER=9c4a48aa66e652]
      --smtp-pass <SMTP_PASS>
          SMTP credencials pass [env: SMTP_PASS=b2a9416a823c93]
  -h, --help
          Print help
  -V, --version
          Print version
```

To facilitate running the application, all command line arguments can be retrieved from environment variables. For example, --repo is REPO env var, and --smtp-host will be taken from SMTP_HOST. Use .env file for convenience. The example file is in the root of the project (.example.env) and can serve you as a template. Just copy it into .env and add your values.

Suppose all SMTP and GitHub settings are stored in .env file; it's trivial to run the app:

```
./target/release/pr-weekly -r rails/rails -s chikolad@gmail.com
```

## SMTP Provide Configuration

You'll need working SMTP provider configuration for the app to send emails. I used Mailtrap (https://mailtrap.io/) service for testing and Postmark (https://postmarkapp.com/) for production usage.

