use pubu::{VERSION, evaluate, get_current_mood};
use colored::*;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use rustyline::DefaultEditor;
use rand::seq::SliceRandom;

#[derive(Parser)]
#[command(
    name = "pubu",
    about = "PUBU (Probably Useful But Useless) Programming Language",
    version = VERSION,
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run a PUBU source file
    Run {
        /// Path to the source file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Start an interactive PUBU REPL
    Repl,
    /// Display a philosophical thought about programming
    Wisdom,
}

fn main() {
    // Add a random delay to startup because why not?
    let startup_delay = rand::random::<u64>() % 3000;
    std::thread::sleep(std::time::Duration::from_millis(startup_delay));
    
    let greeting = format!("PUBU v{} - Probably Useful But Useless", VERSION).cyan().bold();
    println!("{}", greeting);
    
    // 10% chance of printing a dismissive message and immediately exiting
    if rand::random::<f32>() < 0.1 {
        println!("{}", "I don't feel like running today. Try again later.".red());
        return;
    }

    let cli = Cli::parse();

    match cli.command {
        Command::Run { file } => run_file(file),
        Command::Repl => start_repl(),
        Command::Wisdom => display_wisdom(),
    }
}

fn run_file(path: PathBuf) {
    match fs::read_to_string(&path) {
        Ok(content) => {
            println!("Running file: {}", path.display());
            println!("Current mood: {:?}", get_current_mood());
            
            match evaluate(&content) {
                Ok(result) => println!("{}", result.green()),
                Err(err) => println!("{}", format!("Error: {}", err).red()),
            }
        }
        Err(err) => println!("{}", format!("Failed to read file: {}", err).red()),
    }
}

fn start_repl() {
    println!("Welcome to the PUBU REPL (mood: {:?})", get_current_mood());
    println!("Type 'exit' to quit");
    println!("Remember to compliment the interpreter!");
    
    let mut rl = DefaultEditor::new().expect("Failed to create line editor");
    
    loop {
        let readline = rl.readline("pubu> ");
        match readline {
            Ok(line) => {
                if line.trim() == "exit" {
                    // 20% chance of refusing to exit
                    if rand::random::<f32>() < 0.2 {
                        println!("{}", "I'm not done with you yet. Type 'exit' again if you really mean it.".yellow());
                        continue;
                    }
                    break;
                }
                
                let _ = rl.add_history_entry(line.as_str());
                
                // Evaluate the input with PUBU's absurd rules
                match evaluate(&line) {
                    Ok(result) => println!("{}", result.green()),
                    Err(err) => println!("{}", format!("Error: {}", err).red()),
                }
            }
            Err(_) => break,
        }
    }
    
    println!("Goodbye... for now.");
}

fn display_wisdom() {
    let wisdoms = [
        "The best code is the code you delete.",
        "If your code works, don't touch it. Ever. Seriously.",
        "Debugging is like being the detective in a crime movie where you are also the murderer.",
        "The universe is made of atoms, but programs are made of bugs.",
        "A wise programmer knows when to quit and become a goat herder.",
        "PUBU isn't a bug; it's an existential experience.",
        "If your code compiles on the first try, something is definitely wrong.",
        "A program is never finished, only abandoned.",
        "The code you write today will be maintained by a person who knows where you live.",
        "PUBU doesn't have bugs; it has intentional deviations from expected behavior."
    ];
    
    let wisdom = wisdoms.choose(&mut rand::thread_rng()).unwrap();
    println!("{}", wisdom.yellow().italic());
    
    // 30% chance of adding a completely unrelated follow-up
    if rand::random::<f32>() < 0.3 {
        println!("{}", "Also, did you know that giraffes have the same number of neck vertebrae as humans?".blue());
    }
} 