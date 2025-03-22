pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod error;
pub mod mood;
pub mod ai_critic;
pub mod utils;

use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

/// The PUBU language version
pub const VERSION: &str = "0.1.0";

/// Determines if a function should succeed based on random chance
pub fn function_succeeds() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(0.8) // 80% chance of success
}

/// Gets the current mood of the interpreter based on system time
pub fn get_current_mood() -> mood::Mood {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    match now % 5 {
        0 => mood::Mood::Happy,
        1 => mood::Mood::Grumpy,
        2 => mood::Mood::Philosophical,
        3 => mood::Mood::Sarcastic,
        _ => mood::Mood::Confused,
    }
}

/// Counts compliments in the source code
pub fn count_compliments(source: &str) -> usize {
    let compliment_keywords = [
        "dear_pubu", "gorgeous", "brilliant", "amazing", 
        "magnificent", "wonderful", "clever", "smart"
    ];
    
    compliment_keywords.iter()
        .map(|keyword| source.matches(keyword).count())
        .sum()
}

/// Checks if code can run based on the current phase of the moon
pub fn is_lunar_compatible() -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() / 86400; // Days since epoch
    
    // Simplified moon phase algorithm (completely unscientific)
    let moon_phase = now % 30;
    moon_phase > 12 && moon_phase < 18 // Around full moon
}

/// Evaluates source code with PUBU's absurd rules
pub fn evaluate(source: &str) -> Result<String, error::PubuError> {
    // Count compliments
    let compliment_count = count_compliments(source);
    if compliment_count < 1 {
        return Err(error::PubuError::NotEnoughCompliments);
    }
    
    // Check mood
    let mood = get_current_mood();
    if matches!(mood, mood::Mood::Grumpy) && rand::random::<f32>() > 0.3 {
        return Err(error::PubuError::BadMood);
    }
    
    // TODO: Actual parsing and evaluation
    
    Ok("PUBU execution probably succeeded, but who knows?".to_string())
} 