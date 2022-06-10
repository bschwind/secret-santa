use rand::{seq::SliceRandom, thread_rng};
use sendgrid::v3::{Content, Email, Message, Personalization, Sender};
use serde::Deserialize;

const FROM_EMAIL: &str = "sender@example.com";

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct Person {
    name: String,
    email: String,
    last_year_giftee: String,
}

fn pairs_are_valid(pairs: &[(Person, Person)]) -> bool {
    pairs
        .iter()
        .all(|(gifter, recipient)| gifter != recipient && gifter.last_year_giftee != recipient.name)
}

fn email_gifter(sender: &Sender, (gifter, recipient): &(Person, Person)) {
    let to_email = Email::new(&gifter.email);
    let from_email = Email::new(FROM_EMAIL);

    let personalization = Personalization::new(to_email);

    let msg = Message::new(from_email)
        .set_subject(&format!("🎅❄️ Hi {}, your Secret Santa giftee is here 🎁🤶", gifter.name))
        .add_content(
            Content::new()
                .set_content_type("text/plain")
                .set_value(&format!("Your secret santa giftee is {}", recipient.name)),
        )
        .add_personalization(personalization);

    let response = sender.send(&msg);

    println!("Sent to {}, response was {:?}", gifter.email, response);
}

fn main() {
    let emails_json_str = include_str!("emails.json");
    let mut people: Vec<Person> = serde_json::from_str(emails_json_str).unwrap();
    let original_people = people.clone();

    let mut rng = thread_rng();
    // Use this if you want a deterministic run every time, make up a number and keep it for
    // later so you can resend if needed
    // let mut rng = StdRng::seed_from_u64(12345);

    // Shuffle the recipients until we have valid pairs
    let pairs: Vec<(Person, Person)> = loop {
        people.shuffle(&mut rng);

        let potential_pairs: Vec<_> =
            original_people.clone().into_iter().zip(people.clone()).collect();

        if pairs_are_valid(&potential_pairs) {
            break potential_pairs;
        }
    };

    let api_key = include_str!("sendgrid_api_key.txt");
    let sender = Sender::new(api_key.to_string());

    for pair in pairs {
        email_gifter(&sender, &pair);
    }
}
