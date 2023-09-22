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
}
