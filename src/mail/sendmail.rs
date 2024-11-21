use std::{env, fs};
use lettre::{
    message::{header, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport,
    Transport
};

pub async fn send_email(
    to_email: &str,
    subject: &str,
    template_path: &str,
    placeholders: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {

    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME_NOT_FOUND");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD_NOT_FOUND");
    let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER_NOT_FOUND");
    let smtp_port = env::var("SMTP_PORT")?.parse().expect("SMTP_PORT_NOT_FOUND");

    let mut html_template = fs::read_to_string(template_path)?;

    for(key, value) in placeholders{
        html_template = html_template.replace(key, value);
    }

    let email = Message::builder()
        .from(smtp_username.parse()?)
        .to(to_email.parse()?)
        .header(header::ContentType::TEXT_HTML)
        .singlepart(SinglePart::builder()
            .header(header::ContentType::TEXT_HTML)
            .body(html_template))?;

    let creds = Credentials::new(smtp_username, smtp_password);
    let mailer = SmtpTransport::starttls_relay(&smtp_server)?
        .credentials(creds)
        .port(smtp_port)
        .build();

    let result = mailer.send(&email);

    match result {
        Ok(_) => println!("Email sent successfully!"),
        Err(error) => println!("Email sent with error: {:?}", error)
    }

    Ok(())
}