use std::sync::Arc;

use anyhow::Result;
use handlebars::Handlebars;
use lettre::{
    Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    Transport as _,
    message::{Mailbox, header::ContentType},
    transport::smtp::{authentication::Credentials, response::Response},
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::info;
use walkdir::WalkDir;

#[derive(Clone)]
pub struct Emailer
{
    pub creds: Credentials,
    pub mailer: AsyncSmtpTransport<Tokio1Executor>,
    pub from_email: Mailbox,
    pub templates: Arc<RwLock<Handlebars<'static>>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SMTP
{
    pub password: String,
    pub username: String,
    // pub port: u16,
    pub server: String,
    pub from: Address,
}
impl Emailer
{
    pub fn init(config: Option<SMTP>) -> Option<Emailer>
    {
        if let Some(config) = config {
            let mut templates: Handlebars<'_> = Handlebars::new();
            let mut walkdir =
                WalkDir::new("./templates").max_depth(1).into_iter();
            while let Some(Ok(entry)) = walkdir.next() {
                if entry.file_type().is_file() {
                    let content =
                        std::fs::read_to_string(entry.path()).unwrap();
                    let file_name =
                        entry.file_name().to_str().unwrap().to_string();
                    templates
                        .register_template_string(&file_name, content)
                        .unwrap();
                }
            }

            let creds = Credentials::new(config.username, config.password);
            let mailer: AsyncSmtpTransport<Tokio1Executor> =
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(
                    &config.server,
                )
                .unwrap()
                //.port(smtp_settings.port)
                .credentials(creds.to_owned())
                .build();

            return Some(Self {
                creds,
                mailer,
                templates: Arc::new(RwLock::new(templates)),
                from_email: Mailbox::new(None, config.from),
            });
        }
        None
    }
    pub async fn send<D: Serialize>(
        &self,
        to: Mailbox,
        subject: &str,
        template: &str,
        data: D,
    ) -> Result<Response>
    {
        info!("Sending email!");
        let templates = self.templates.read().await;
        let msg = templates.render(template, &data)?;
        return self.internal_send(to, subject.to_string(), msg).await;
    }
    pub async fn internal_send(
        &self,
        to: Mailbox,
        subject: String,
        content: String,
    ) -> Result<Response>
    {
        info!("Sending email! (internal)");
        let msg: std::result::Result<Response, lettre::transport::smtp::Error> =
            self.mailer
                .send(
                    Message::builder()
                        .from(self.from_email.to_owned())
                        .to(to)
                        .subject(subject)
                        .header(ContentType::TEXT_PLAIN)
                        .body(content)?,
                )
                .await;
        info!("Email sent!");
        Ok(msg?)
    }
}
