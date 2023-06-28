use lettre::{
    message::{Mailbox, MultiPart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use crate::{config::Config, errors::Error};

pub struct Address {
    pub name: String,
    pub email_address: String,
}

pub struct Email {
    pub recipient: Address,
    pub sender: Address,
    pub subject: String,
    pub text: String,
    pub html: String,
}

pub fn send_email(config: &Config, email: Email) -> Result<(), Error> {
    let from = mailbox_from_address(email.sender)?;
    let to = mailbox_from_address(email.recipient)?;
    let message = Message::builder()
        .from(from)
        .to(to)
        .subject(email.subject)
        .multipart(MultiPart::alternative_plain_html(email.text, email.html))
        .map_err(|_| Error::InternalServerError(format!("Could not create email message")))?;
    let transport = transport_from_config(config)?;
    transport
        .send(&message)
        .map_err(|_| Error::InternalServerError(format!("Could not send email")))?;
    Ok(())
}

pub fn mailbox_from_address(address: Address) -> Result<Mailbox, Error> {
    format!("{} <{}>", address.name, address.email_address)
        .parse::<Mailbox>()
        .map_err(|_| Error::InternalServerError(format!("Could not email user")))
}

pub fn transport_from_config(config: &Config) -> Result<SmtpTransport, Error> {
    Ok(SmtpTransport::starttls_relay(&config.email.smtp_host)
        .map_err(|_| Error::InternalServerError(format!("Could not connect to email server")))?
        .port(config.email.smtp_port)
        .credentials(Credentials::new(
            config.email.smtp_user.clone(),
            config.email.smtp_password.clone(),
        ))
        .build())
}
