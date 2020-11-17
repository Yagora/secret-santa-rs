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
    let config_file = File::open("config.yaml").expect("Can't open file");
    let mut config: Config = serde_yaml::from_reader(config_file).expect("Can't parse opened file");
    let mut body_template = TinyTemplate::new();

    config.mails_to.shuffle(&mut thread_rng());

    body_template
        .add_template("body", &config.body)
        .expect("Can't add body to the template");

    for (i, mail_to) in config.mails_to.iter().enumerate() {
        let mut receiving_gift_index = i + 1;

        // take the first mail_to if the current is the last
        if i == config.mails_to.len() - 1 {
            receiving_gift_index = 0;
        }

        let chunks_receiving_mail: Vec<&str> =
            config.mails_to[receiving_gift_index].split(' ').collect();
        let context = Context {
            first_name: chunks_receiving_mail[0].to_string(),
            last_name: chunks_receiving_mail[1].to_string(),
        };

        let email = Message::builder()
            .from(config.mail_from.parse().expect("Can't parse mail_from"))
            .to(mail_to.parse().expect("Can't parse mail_to"))
            .subject(config.subject.clone())
            .body(
                body_template
                    .render("body", &context)
                    .expect("Unknown template body")
                    .clone(),
            )
            .expect("Can't build message");

        let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

        // Open a remote connection
        let mailer = SmtpTransport::relay(&config.smtp_server)
            .expect("Can't upgrad the connection")
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
}
