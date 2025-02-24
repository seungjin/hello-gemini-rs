// main.rs
use gemini_rs::Conversation;

#[tokio::main]
async fn main() {
    let mut convo = Conversation::new(
        std::env::var("GEMINI_API_KEY").unwrap(), // Replace with however you want to get your API key
        "gemini-1.5-flash".to_string() // Replace with the desired model from https://ai.google.dev/gemini-api/docs/models/gemini
    );

    let a = convo.prompt("If you had to describe Risk of Rain 2 in one word, what word would it be?").await;
    println!("{a}");
    let b = convo.prompt("Now explain your reasoning").await;
    println!("{b}");
}


