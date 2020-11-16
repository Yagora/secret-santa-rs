use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde_yaml;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;

fn main() {
    let config_file = std::fs::File::open("config.yaml").unwrap();
    let config_file_btree: BTreeMap<String, String> = serde_yaml::from_reader(config_file).unwrap();
    let smtp_server = config_file_btree.get("smtp_server").as_deref().unwrap().to_string();
    let smtp_username = config_file_btree.get("smtp_username").as_deref().unwrap().to_string();
    let smtp_password = config_file_btree.get("smtp_password").as_deref().unwrap().to_string();
    let mail_from = config_file_btree.get("mail_from").as_deref().unwrap().to_string();
    let mail_to = config_file_btree.get("mail_to").as_deref().unwrap().to_string();
    let subject = config_file_btree.get("subject").as_deref().unwrap().to_string();
    let body = config_file_btree.get("body").as_deref().unwrap().to_string();

    let email = Message::builder()
        .from(mail_from.parse().unwrap())
        .to(mail_to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_password);

    // Open a remote connection
    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
