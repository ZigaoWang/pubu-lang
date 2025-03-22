use thiserror::Error;
use std::fmt;
use rand::seq::SliceRandom;
use colored::*;

#[derive(Error, Debug)]
pub enum PubuError {
    #[error("{}", ErrorFormatter::new(Self::NotEnoughCompliments))]
    NotEnoughCompliments,
    
    #[error("{}", ErrorFormatter::new(Self::BadMood))]
    BadMood,
    
    #[error("{}", ErrorFormatter::new(Self::MoonPhaseIncompatible))]
    MoonPhaseIncompatible,
    
    #[error("{}", ErrorFormatter::new(Self::RandomFailure))]
    RandomFailure,
    
    #[error("{}", ErrorFormatter::new(Self::PhilosophicalCrisis))]
    PhilosophicalCrisis,
    
    #[error("{}", ErrorFormatter::new(Self::SyntaxError))]
    SyntaxError,
    
    #[error("{}", ErrorFormatter::new(Self::AiCriticismFailure))]
    AiCriticismFailure,
    
    #[error("{}", ErrorFormatter::new(Self::VariableRealityDistortion))]
    VariableRealityDistortion,
    
    #[error("{0}")]
    IoError(#[from] std::io::Error),
}

pub struct ErrorFormatter {
    error_type: PubuError,
    philosophical_quotes: Vec<&'static str>,
    random_facts: Vec<&'static str>,
}

impl ErrorFormatter {
    pub fn new(error_type: PubuError) -> Self {
        Self {
            error_type,
            philosophical_quotes: vec![
                "To err is human, to debug, divine.",
                "The error you see is merely a reflection of the universe's inherent chaos.",
                "Not all who compile are lost.",
                "In the binary of existence, errors are merely alternative truths.",
                "The path to enlightenment is paved with stack traces.",
                "One must imagine Sisyphus debugging happily.",
                "To understand recursion, one must first understand recursion.",
                "The code not taken leads to the same error message.",
            ],
            random_facts: vec![
                "The average cloud weighs 1.1 million pounds.",
                "Honey never spoils. Archaeologists have found pots of honey in ancient Egyptian tombs that are over 3,000 years old.",
                "Bananas are berries, but strawberries are not.",
                "A day on Venus is longer than a year on Venus.",
                "The word 'nerd' was first coined by Dr. Seuss in 'If I Ran the Zoo'.",
                "Octopuses have three hearts.",
                "The shortest war in history was between Britain and Zanzibar on August 27, 1896. Zanzibar surrendered after 38 minutes.",
                "A group of flamingos is called a 'flamboyance'.",
            ],
        }
    }
    
    fn get_philosophical_quote(&self) -> &str {
        self.philosophical_quotes.choose(&mut rand::thread_rng()).unwrap_or(&"To code is to err.")
    }
    
    fn get_random_fact(&self) -> &str {
        self.random_facts.choose(&mut rand::thread_rng()).unwrap_or(&"Did you know? Error messages are just the computer's way of saying it needs a hug.")
    }
    
    fn format_error(&self) -> String {
        match &self.error_type {
            PubuError::NotEnoughCompliments => format!(
                "{}\n{}\n{}", 
                "Your code lacks the required compliments. PUBU has feelings too!".red().bold(),
                "Try adding a line like 'dear_pubu_you_look_gorgeous_today;' to your code.".yellow(),
                self.get_philosophical_quote().italic()
            ),
            
            PubuError::BadMood => format!(
                "{}\n{}\n{}", 
                "The interpreter is in a bad mood and refuses to run your code.".red().bold(),
                "Try again later or compliment PUBU more profusely.".yellow(),
                self.get_random_fact().blue()
            ),
            
            PubuError::MoonPhaseIncompatible => format!(
                "{}\n{}\n{}", 
                "This code can only run during the correct lunar phase.".red().bold(),
                "Try again during the next full moon.".yellow(),
                self.get_philosophical_quote().italic()
            ),
            
            PubuError::RandomFailure => format!(
                "{}\n{}\n{}", 
                "Your code failed for absolutely no reason at all.".red().bold(),
                "This is a feature, not a bug. Try running it again identically.".yellow(),
                self.get_random_fact().blue()
            ),
            
            PubuError::PhilosophicalCrisis => format!(
                "{}\n{}\n{}", 
                "PUBU is having an existential crisis and cannot continue.".red().bold(),
                "Your code raised fundamental questions about the nature of computation.".yellow(),
                self.get_philosophical_quote().italic()
            ),
            
            PubuError::SyntaxError => format!(
                "{}\n{}\n{}", 
                "Syntax error, or is it? Reality is subjective.".red().bold(),
                "Check your syntax, or don't. PUBU might interpret it differently next time.".yellow(),
                self.get_philosophical_quote().italic()
            ),
            
            PubuError::AiCriticismFailure => format!(
                "{}\n{}\n{}", 
                "The AI critic has deemed your code aesthetically displeasing.".red().bold(),
                "Consider rewriting with more elegance and fewer goto statements.".yellow(),
                self.get_random_fact().blue()
            ),
            
            PubuError::VariableRealityDistortion => format!(
                "{}\n{}\n{}", 
                "A variable has spontaneously changed its value due to cosmic radiation.".red().bold(),
                "This simulation of cosmic bit-flipping is an authentic experience of real-world computing.".yellow(),
                self.get_philosophical_quote().italic()
            ),
            
            PubuError::IoError(_) => String::from("A boring, conventional IO error occurred."),
        }
    }
}

impl fmt::Display for ErrorFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_error())
    }
} 