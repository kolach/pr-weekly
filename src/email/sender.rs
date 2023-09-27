use super::error::EmailError;
use lettre::{
    message::{header::ContentType, IntoBody, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

#[derive(Debug)]
pub struct Sender {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
}

impl Sender {
    pub fn new(host: String, port: u16, user: String, pass: String) -> Self {
        Self {
            smtp_host: host,
            smtp_port: port,
            smtp_user: user,
            smtp_pass: pass,
        }
    }

    fn new_transport(&self) -> Result<AsyncSmtpTransport<Tokio1Executor>, EmailError> {
        let creds = Credentials::new(self.smtp_user.to_owned(), self.smtp_pass.to_owned());

        let transport =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.smtp_host.to_owned())?
                .port(self.smtp_port)
                .credentials(creds)
                .build();

        Ok(transport)
    }

    pub async fn send<T>(
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
            .header(ContentType::TEXT_HTML)
            .from(from.to_owned())
            .to(to)
            .reply_to(from)
            .subject(subject)
            .body(content)?;

        let transport = self.new_transport()?;
        transport.send(message).await?;
        Ok(())
    }
}
