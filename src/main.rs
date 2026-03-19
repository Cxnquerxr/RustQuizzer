mod question;
mod storage;
mod quiz;

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use std::{io::{self}};

use question::Question;
use storage::{load_questions, save_questions};

#[derive(Parser)]
#[command(name = "quiz")]
#[command(about = "CLI quiz game", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Enter questions
    Add {
        /// Path to JSON file
        file: String,
    },

    /// Play quiz
    Play {
        /// Path to JSON file
        file: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { file } => add_mode(&file)?,
        Commands::Play { file } => play_mode(&file)?,
    }

    Ok(())
}

fn add_mode(path: &str) -> Result<()> {
    let mut questions = load_questions(path).unwrap_or_else(|_| Vec::new());

    let stdin = io::stdin();
    let mut buffer = String::new();
    loop {
        println!("\nEnter question:");
        stdin.read_line(&mut buffer);
        let prompt = buffer.trim().to_string();

        let mut options = Vec::new();
        for i in 1..=4 {
            buffer.clear();
            println!("Option {}:", i);
            stdin.read_line(&mut buffer);
            options.push(buffer.trim().to_string());
        }

        buffer.clear();
        println!("Correct answer (1-4):");
        stdin.read_line(&mut buffer);
        let correct_input = buffer.trim().to_string();
        let correct_index: usize = correct_input.parse()
            .map_err(|_| anyhow::anyhow!("Invalid number"))?;

        if correct_index < 1 || correct_index > 4 {
            bail!("Answer must be between 1 and 4");
        }

        let question = Question {
            prompt,
            options,
            correct_index: correct_index - 1,
        };

        question.validate()?;
        questions.push(question);

        buffer.clear();
        println!("Add another? (y/n)");
        stdin.read_line(&mut buffer);
        let again = buffer.trim().to_string();
        if again.to_lowercase() != "y" {
            break;
        }
        buffer.clear();
    }

    save_questions(path, &questions)?;
    println!("Questions saved.");

    Ok(())
}

fn play_mode(path: &str) -> Result<()> {
    let questions = load_questions(path)?;
    let stdin = io::stdin();
    let mut buffer = String::new();

    if questions.is_empty() {
        bail!("No questions found.");
    }

    let mut score = 0;

    for (i, q) in questions.iter().enumerate() {
        println!("\nQuestion {}:", i + 1);
        println!("{}", q.prompt);

        for (idx, opt) in q.options.iter().enumerate() {
            println!("{}. {}", idx + 1, opt);
        }

        println!("Your answer:");
        stdin.read_line(&mut buffer);
        let input = buffer.trim().to_string();

        let answer: usize = match input.parse::<usize>() {
            Ok(n) if n >= 1 && n <= 4 => n - 1,
            _ => {
                println!("Invalid input, skipping question.");
                continue;
            }
        };

        if answer == q.correct_index {
            println!("Correct!");
            score += 1;
        } else {
            println!("Wrong!");
        }
    }

    println!("\nFinal score: {}/{}", score, questions.len());

    Ok(())
}