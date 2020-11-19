use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::fs::File;
use tinytemplate::TinyTemplate;

#[derive(Debug, Deserialize)]
struct Config {
    smtp_server: String,
    smtp_username: String,
    smtp_password: String,
    mail_from: String,
    mails_to: Vec<String>,
    subject: String,
    body: String,
}
#[derive(Serialize)]
struct Context {
    first_name: String,
    last_name: String,
}


fn main() {
    let config_file = File::open("config.yaml").expect("Cannot open the file");
    let mut config: Config = serde_yaml::from_reader(config_file).expect("Cannot parse the config file");
    let mut body_template = TinyTemplate::new();

    config.mails_to.shuffle(&mut thread_rng());

    body_template
        .add_template("body", &config.body)
        .expect("Cannot add body to the template");

    for (i, mail_to) in config.mails_to.iter().enumerate() {
        let mut receiver_gift_index: usize = i + 1;

        // take the first index if it's the last email
        if i == config.mails_to.len() - 1 {
            receiver_gift_index = 0;
        }

        let chunks_receiver_mail: Vec<&str> = config.mails_to[receiver_gift_index].split(' ').collect();
        let context = Context {
            first_name: chunks_receiver_mail[0].to_string(),
            last_name: chunks_receiver_mail[1].to_string()
        };

        let email = Message::builder()
            .from(config.mail_from.parse().expect("Cannot parse mail_from"))
            .to(mail_to.parse().expect("Cannot parse mail_to"))
            .subject(config.subject.clone())
            .body(
                body_template
                    .render("body", &context)
                    .expect("Cannot render the body template")
                    .clone()
            )
            .expect("Cannot build the email");

        let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

        // Open a remote connection
        let mailer = SmtpTransport::relay(&config.smtp_server)
            .expect("Cannot connect to smtp server")
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
}
