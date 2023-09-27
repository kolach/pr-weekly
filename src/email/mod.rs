mod error;
mod sender;

pub use error::EmailError;
pub use sender::Sender;

use handlebars::Handlebars;
use rust_embed::RustEmbed;
use serde::Serialize;

#[derive(RustEmbed)]
#[folder = "src/email/templates"]
struct Assets;

pub fn render<T>(subject: &str, pulls: T) -> Result<String, EmailError>
where
    T: Serialize,
{
    let mut hbs = Handlebars::new();
    hbs.register_embed_templates::<Assets>()?;
    hbs.register_escape_fn(handlebars::no_escape);

    let data = serde_json::json!({
        "subject": subject,
        "summary": pulls,
    });

    let content = hbs.render("summary.hbs", &data)?;

    Ok(content)
}
