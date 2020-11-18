use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde_yaml;
use serde::Deserialize;
use serde::Serialize;
use tinytemplate::TinyTemplate;

use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug, Deserialize)]
struct Config {
    smtp_server: String,
    smtp_username: String,
    smtp_password: String,
    mail_from: String,
    mails_to: Vec<String>,
    subject: String,
    body: String
}
#[derive(Serialize)]
struct Context {
    first_name: String,
    last_name: String
}


fn main() {
    let config_file = std::fs::File::open("config.yaml").unwrap();
    let config_file_btree: Config = serde_yaml::from_reader(config_file).unwrap();
    let mut body_template = TinyTemplate::new();


    let mut mailts_to: Vec<String> = config_file_btree.mails_to;
    mailts_to.shuffle(&mut thread_rng());

    body_template.add_template("body", &config_file_btree.body).unwrap();

    for (i, mail_to) in mailts_to.iter().enumerate() {
        let mut receiver_gift_index: usize = i;

        // used to take the next mail to defined the person who receives the gift if it is the last mail get the first Vec's entry
        if i == mailts_to.len() - 1 {
            receiver_gift_index = 0;
        }

        let chunks_receiver_mail: Vec<&str> = mailts_to[receiver_gift_index].split(" ").collect();
        let context = Context {
            first_name: chunks_receiver_mail[0].to_string(),
            last_name: chunks_receiver_mail[1].to_string()
        };

        let email = Message::builder()
            .from(config_file_btree.mail_from.parse().unwrap())
            .to(mail_to.parse().unwrap())
            .subject(config_file_btree.subject.to_string())
            .body(body_template.render("body", &context).unwrap().to_string())
            .unwrap();

        let creds = Credentials::new(config_file_btree.smtp_username.to_string(), config_file_btree.smtp_password.to_string());

        // Open a remote connection
        let mailer = SmtpTransport::relay(&config_file_btree.smtp_server)
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
}
