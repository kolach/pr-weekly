use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Templates error: {source:?}")]
    TemplateError {
        #[from]
        source: handlebars::TemplateError,
    },
    #[error("TemplateRender error: {source:?}")]
    TemplateRenderError {
        #[from]
        source: handlebars::RenderError,
    },

    #[error("Transport SMTP error: {source:?}")]
    TransportSmtpError {
        #[from]
        source: lettre::transport::smtp::Error,
    },

    #[error("Email error: {source:?}")]
    EmailError {
        #[from]
        source: lettre::error::Error,
    },
}
