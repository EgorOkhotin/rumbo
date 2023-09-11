use super::Result;
use ureq::Response;
use serde_json::json;
use serde_json::{Map, Value};
use lettre::message::header::ContentType;
use lettre::{Message,SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;


#[derive(Debug)]
pub struct SmtpCredential{
    username: String,
    password: String
}

impl SmtpCredential {
    pub fn new(username: String, password: String) -> Self{
        Self{username, password}
    }

    pub fn to(self)->Credentials{
        Credentials::new(self.username, self.password)
    }
}

impl From<(String, String)> for SmtpCredential{
    fn from(value: (String, String)) -> Self {
        Self { username: value.0, password: value.1 }
    }
}

#[derive(Debug)]
pub struct EmailAddress<'a>{
    to: &'a str,
    from: &'a str,
    reply_to: Option<&'a str>
}

impl<'a> EmailAddress<'a>{
    pub fn new(to: &'a str, from: &'a str, reply_to: Option<&'a str>) -> Self{
        Self{to, from, reply_to}
    }
}

#[derive(Debug)]
pub struct EmailContent<'a>{
    body: String,
    subject: &'a str
}

impl<'a> EmailContent<'a>{
    pub fn new(subject: &'a str, body: String) -> Self{
        Self{subject, body}
    }
}

impl<'a> From<(&'a str, String)> for EmailContent<'a>{
    fn from(value: (&'a str, String)) -> Self {
        Self{subject: value.0, body: value.1}
    }
}

pub fn send_smtp_email(credential:SmtpCredential, address:EmailAddress, content:EmailContent)->Result<()>{
    let mut email = Message::builder()
    .from(address.from.parse()?)
    .to(address.to.parse()?)
    .subject(content.subject)
    .header(ContentType::TEXT_PLAIN);
    if let Some(address) =  address.reply_to{
        email = email.reply_to(address.parse()?);
    }

    let message = email.body(content.body)?;

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(credential.to())
        .build();

    // Send the email
    mailer.send(&message)?;
    Ok(())
}

pub fn send_telegram_message(msg: String, token: &str, chat_id: i64) -> Result<Response> {
    let mut request_body = Map::new();
    request_body.insert("text".to_string(), Value::String(msg));
    request_body.insert("chat_id".to_string(), json!(chat_id));
    request_body.insert(
        "parse_mode".to_string(), Value::String("MarkdownV2".to_string())
    );

    let resp = ureq::post(&format!(
        "https://api.telegram.org/bot{token}/sendMessage", token = &token
    ))
    .send_json(json!(request_body))?;
    Ok(resp)
}
