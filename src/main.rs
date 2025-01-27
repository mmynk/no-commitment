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
        Ok(answer) => println!("{} says: {}", T::name(), answer),
        Err(e) => println!("{} failed: {}", T::name(), e.message),
    }
}
