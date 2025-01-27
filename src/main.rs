use clap::Parser;
use std::{
    fs, future,
    io::{self, Read},
};

pub mod curl;
pub mod deepseek;
pub mod error;
pub mod gemini;

/// Generate commit messages from `git diff` output
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// AI to ask
    #[arg(short, long, default_value = "gemini")]
    ai: String,
}

fn load_env() {
    let env_vars = fs::read_to_string(".env");
    if env_vars.is_err() {
        println!("No .env file found");
        return;
    }
    let envs = env_vars.unwrap();
    for line in envs.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            continue;
        }
        let key = parts[0];
        let value = parts[1];
        std::env::set_var(key, value);
    }
}

pub trait Ask {
    fn name() -> &'static str;

    fn prompt() -> &'static str {
        "You are a terminal based AI assistant.\
            You are supposed to generate commit messages.\
            Your input is the output of `git diff` command.\
            Generate a commit message summarizing the major changes based on the Git diff. Keep the message concise and to the point. Output multi-line messages only for complex or significant changes, and avoid detailing trivial changes like whitespace adjustments or minor refactorings. Focus on the major impact of the changes, such as new features, bug fixes, or larger refactorings.\
            For simple diffs, generate a single line commit message.\
            For complex diffs, generate a multi-line commit message in the format:\
            A brief summary of the changes.\n\
            \n\
            - A detailed description of the changes.\
            \n"
    }

    fn ask(diff: &str) -> impl future::Future<Output = Result<String, error::Error>>;
}

#[tokio::main]
async fn main() {
    load_env();

    let args = Args::parse();
    let ai = args.ai.to_lowercase();

    let mut diff = String::new();

    if atty::isnt(atty::Stream::Stdin) {
        io::stdin()
            .read_to_string(&mut diff)
            .expect("Failed to read from stdin");
    } else {
        eprintln!("Error: Input must be piped or provided as an argument.");
        std::process::exit(1);
    }

    match ai.as_str() {
        "deepseek" => answer::<deepseek::Deepseek>(&diff).await,
        "gemini" => answer::<gemini::Gemini>(&diff).await,
        _ => println!("Unknown AI: {}", ai),
    }
}

async fn answer<T: Ask>(diff: &str) {
    let response = T::ask(diff).await;
    match response {
        Ok(answer) => println!("{}", answer),
        Err(e) => println!("{} failed: {}", T::name(), e.message),
    }
}
