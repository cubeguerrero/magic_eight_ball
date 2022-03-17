use std::error::Error;
use std::io;
use rand::Rng;

static ANSWERS: [&str; 20] = [
    "It is certain.",
    "It is decidedly so.",
    "Without a doubt.",
    "Yes definitely.",
    "You may rely on it.",
    "As I see it, yes.",
    "Most likely.",
    "Outlook good.",
    "Yes",
    "Signs point to yes",
    "Reply hazy, try again.",
    "Ask again later",
    "Better not tell you now.",
    "Cannot predict now.",
    "Concentrate and ask again",
    "Don't count on it.",
    "My reply is no.",
    "My sources say no.",
    "Outlook not so good.",
    "Very doubtful",
];

fn main() -> Result<(), Box<dyn Error>>{
    println!("Welcome to the Magic 8 Ball!");
    println!("Ask your question: ");

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let mut rng = rand::thread_rng();
    println!("{}", ANSWERS[rng.gen_range(0..20)]);
    Ok(())
}
