use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn main() {
    let email = Message::builder()
        .from("Tang <tanguy.charon@donow.in>".parse().unwrap())
        .to("Tang <contact@donow.in>".parse().unwrap())
        .subject("Sent from rust program")
        .body("On ne va pas en ruster la")
        .unwrap();

    let creds = Credentials::new("tanguy.charon@donow.in".to_string(), "sdfdf".to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("ssl0.ovh.net")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
