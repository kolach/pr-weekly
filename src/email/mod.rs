mod client;
mod error;

pub use error::EmailError;
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use serde::Serialize;

#[derive(RustEmbed)]
#[folder = "src/email/templates"]
struct Assets;

pub fn render<T>(pulls: &Vec<T>) -> Result<String, EmailError>
where
    T: Serialize,
{
    let mut hbs = Handlebars::new();
    hbs.register_embed_templates::<Assets>()?;
    hbs.register_escape_fn(handlebars::no_escape);

    let data = serde_json::json!({
        "subject": "Pull Requests",
        "pulls": pulls,
    });

    let content = hbs.render("pulls.hbs", &data)?;

    Ok(content)
}
