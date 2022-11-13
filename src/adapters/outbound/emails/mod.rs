pub mod sendgrid;

use async_trait::async_trait;

#[async_trait]
pub trait EmailSender {
    async fn send_email(
        from: &String,
        to: &Vec<String>,
        subject: &String,
        content: &String,
    ) -> Result<(), ()>;
}
