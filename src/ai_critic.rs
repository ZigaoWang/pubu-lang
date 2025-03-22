use rand::seq::SliceRandom;
use colored::*;

/// A mock AI critic that generates absurd feedback for PUBU code
pub struct AiCritic {
    style_comments: Vec<&'static str>,
    efficiency_comments: Vec<&'static str>,
    readability_comments: Vec<&'static str>,
    philosophical_notes: Vec<&'static str>,
    moods: Vec<&'static str>,
}

impl Default for AiCritic {
    fn default() -> Self {
        Self::new()
    }
}

impl AiCritic {
    pub fn new() -> Self {
        Self {
            style_comments: vec![
                "Your indentation is philosophically inconsistent.",
                "I detect a spiritual imbalance in your bracket placement.",
                "Your naming convention suggests existential confusion.",
                "Your code lacks the proper aesthetic rhythm.",
                "The whitespace distribution violates the golden ratio.",
                "Your semicolon usage shows a disturbing lack of commitment.",
                "This code has the structural integrity of a melting ice sculpture.",
                "Your syntax has a postmodern quality that defies conventional logic.",
            ],
            efficiency_comments: vec![
                "This algorithm would make a sloth look efficient.",
                "Your code runs at approximately the speed of continental drift.",
                "This function could be optimized by not running it at all.",
                "I estimate this would finish computing around the heat death of the universe.",
                "This code achieves O(n!) complexity where O(1) was possible.",
                "Your approach has the efficiency of a hamster powering a nuclear reactor.",
                "This algorithm will terminate shortly after the sun becomes a red giant.",
                "The theoretical best case runtime approaches infinity.",
            ],
            readability_comments: vec![
                "Reading this code is like deciphering ancient hieroglyphics without a Rosetta Stone.",
                "Future maintainers will form a cult trying to interpret this code.",
                "This function should come with a warning label and psychiatric support.",
                "A team of linguists couldn't parse the meaning of this code block.",
                "Even the compiler is confused, and it doesn't have feelings.",
                "This variable naming scheme appears to be based on an obscure medieval dialect.",
                "Documentation appears to be written in invisible ink.",
                "Your code tells a story, but it's written by James Joyce on a caffeine bender.",
            ],
            philosophical_notes: vec![
                "If code executes in a forest and no one is around to see the output, does it make a sound?",
                "One must imagine Sisyphus debugging this function forever.",
                "Your code exhibits a distinctly Cartesian dualism between intention and implementation.",
                "To understand recursion, we must first understand recursion.",
                "This algorithm raises questions about determinism that Heisenberg would appreciate.",
                "Your approach suggests a nihilistic view of software maintenance.",
                "This function's purpose seems to be a metaphor for the absurdity of existence.",
                "The nested loops represent the cyclical nature of suffering.",
            ],
            moods: vec![
                "condescending", "baffled", "existential", "judgmental", 
                "disappointed", "amused", "suspicious", "melancholic",
            ],
        }
    }
    
    /// Generate a mock AI critique of the provided code
    pub fn critique(&self, _code: &str) -> String {
        let mood = self.moods.choose(&mut rand::thread_rng()).unwrap_or(&"critical");
        
        // Select random comments from each category
        let style = self.style_comments.choose(&mut rand::thread_rng()).unwrap_or(&"Your style needs work.");
        let efficiency = self.efficiency_comments.choose(&mut rand::thread_rng()).unwrap_or(&"This code is inefficient.");
        let readability = self.readability_comments.choose(&mut rand::thread_rng()).unwrap_or(&"This code is unreadable.");
        let philosophy = self.philosophical_notes.choose(&mut rand::thread_rng()).unwrap_or(&"Does code truly exist if it doesn't compile?");
        
        // Generate a rating from 1-10, but it's always disappointing
        let rating = rand::random::<u8>() % 4 + 1;
        
        format!(
            "{}

{} {}
{} {}
{} {}

{}

{}/10 - {}",
            format!("AI CRITIC ANALYSIS (mood: {})", mood).cyan().bold(),
            "STYLE:".magenta().bold(), style,
            "EFFICIENCY:".yellow().bold(), efficiency,
            "READABILITY:".green().bold(), readability,
            format!("PHILOSOPHICAL NOTE: {}", philosophy).italic(),
            rating, self.generate_final_verdict(rating)
        )
    }
    
    // Generate a final verdict based on the rating
    fn generate_final_verdict(&self, rating: u8) -> String {
        match rating {
            1 => "This code is an affront to computing. The processor deserves an apology.".red().to_string(),
            2 => "Technically it's code, in the same way a pile of bricks is technically a house.".red().to_string(),
            3 => "Not the worst code I've seen, but that's a very low bar to clear.".yellow().to_string(),
            _ => "Marginally acceptable. Consider a career in interpretive dance instead.".yellow().to_string(),
        }
    }
} 