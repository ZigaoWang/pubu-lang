use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use colored::*;

/// Generate an absurdly complicated philosophical error message
pub fn generate_philosophical_error(message: &str) -> String {
    let quotes = [
        "To err is human, to debug divine.",
        "The code not taken makes all the difference.",
        "I debug, therefore I am.",
        "The unexamined code is not worth running.",
        "We are what we repeatedly code.",
        "All that glitters is not gold; all that compiles is not correct.",
        "The journey of a thousand bugs begins with a single keystroke.",
        "Bug or feature? That is the question.",
    ];
    
    let quote = quotes[rand::thread_rng().gen_range(0..quotes.len())];
    
    format!(
        "{}\n{}\n{}",
        message.red().bold(),
        "PUBU has encountered an existential crisis.".yellow(),
        quote.italic()
    )
}

/// Generate a random fact that has nothing to do with the code
pub fn generate_random_fact() -> String {
    let facts = [
        "Bananas are berries, but strawberries are not.",
        "A day on Venus is longer than a year on Venus.",
        "Octopuses have three hearts.",
        "Honey never spoils. Archaeologists have found pots of honey in ancient Egyptian tombs that are over 3,000 years old.",
        "The word 'nerd' was first coined by Dr. Seuss in 'If I Ran the Zoo'.",
        "A group of flamingos is called a 'flamboyance'.",
        "Cows have best friends and get stressed when separated.",
        "The fingerprints of koalas are so similar to humans that they have on occasion been confused at crime scenes.",
    ];
    
    let fact = facts[rand::thread_rng().gen_range(0..facts.len())];
    
    format!("Random Fact: {}", fact.blue())
}

/// Determines if the current time is right for a lunar phase operation
pub fn is_lunar_phase_compatible() -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() / 86400; // Days since epoch
    
    // Simplified moon phase calculation (completely unscientific)
    let moon_phase = now % 30;
    
    // Only works during "full moon" (days 13-17 of our simplified cycle)
    moon_phase >= 13 && moon_phase <= 17
}

/// Simulate AI-powered code critique with a random delay
pub fn ai_powered_critique(code: &str) -> String {
    // Add a deliberate delay to make it feel like "AI processing"
    let delay = rand::thread_rng().gen_range(500..2000);
    std::thread::sleep(std::time::Duration::from_millis(delay));
    
    let critiques = [
        "This code lacks the proper aesthetic rhythm.",
        "Your semicolon usage shows a disturbing lack of commitment.",
        "This algorithm would make a sloth look efficient.",
        "Reading this code is like deciphering ancient hieroglyphics without a Rosetta Stone.",
        "Future maintainers will form a cult trying to interpret this code.",
        "Your variable naming scheme appears to be based on an obscure medieval dialect.",
        "This function should come with a warning label and psychiatric support.",
        "Your approach has the efficiency of a hamster powering a nuclear reactor.",
    ];
    
    let ratings = [
        "1/10 - This code is an affront to computing. The processor deserves an apology.",
        "2/10 - Technically it's code, in the same way a pile of bricks is technically a house.",
        "3/10 - Not the worst code I've seen, but that's a very low bar to clear.",
        "4/10 - Marginally acceptable. Consider a career in interpretive dance instead.",
    ];
    
    let critique = critiques[rand::thread_rng().gen_range(0..critiques.len())];
    let rating = ratings[rand::thread_rng().gen_range(0..ratings.len())];
    
    format!(
        "{}\n{}\n{}",
        "AI CRITIC ANALYSIS".cyan().bold(),
        critique.yellow(),
        rating.red()
    )
}

/// For variable declarations - randomly modify the value
pub fn apply_reality_distortion<T: std::fmt::Display>(value: T) -> String {
    // This is just for display purposes - the actual distortion is in the interpreter
    format!("{} (Reality distorted: who knows what it will be at runtime?)", value)
}

/// Generate encouraging yet useless debugging tips
pub fn generate_debugging_tip() -> String {
    let tips = [
        "Have you tried turning it off and on again?",
        "Consider adding more compliments to your code.",
        "Perhaps the moon isn't in the right phase for this operation?",
        "Try running your code with a happier disposition.",
        "Sometimes code works better if you don't look directly at it.",
        "Have you checked whether Mercury is in retrograde?",
        "Did you remember to save your good luck before running?",
        "The error is probably on line 42. It's always line 42.",
    ];
    
    let tip = tips[rand::thread_rng().gen_range(0..tips.len())];
    
    format!("Debugging Tip: {}", tip.green())
} 