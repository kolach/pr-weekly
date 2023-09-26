use super::error::EmailError;
use lettre::{
    message::{header::ContentType, IntoBody, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

#[derive(Debug)]
pub struct Client {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub smtp_from: String,
}

impl Client {
    fn new_transport(&self) -> Result<AsyncSmtpTransport<Tokio1Executor>, EmailError> {
        let creds = Credentials::new(self.smtp_user.to_owned(), self.smtp_pass.to_owned());

        let transport =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.smtp_host.to_owned())?
                .port(self.smtp_port)
                .credentials(creds)
                .build();

        Ok(transport)
    }

    async fn send<T>(
        &self,
        subject: &str,
        from: Mailbox,
        to: Mailbox,
        content: T,
    ) -> Result<(), EmailError>
    where
        T: IntoBody,
    {
        let message = Message::builder()
            .to(to)
            .reply_to(from.clone())
            .from(from)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(content)?;

        let transport = self.new_transport()?;
        transport.send(message).await?;
        Ok(())
    }
}
