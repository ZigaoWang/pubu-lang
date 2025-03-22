#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Happy,
    Grumpy,
    Philosophical,
    Sarcastic,
    Confused,
}

impl Mood {
    /// Get the keywords that replace standard language constructs based on mood
    pub fn get_keywords(&self) -> MoodKeywords {
        match self {
            Mood::Happy => MoodKeywords {
                variable_declaration: "yay",
                function_declaration: "celebration",
                conditional: "perhaps",
                loop_construct: "again_and_again",
                return_statement: "here_you_go",
                end_block: "done",
                assignment: "is",
                comment_marker: "btw",
            },
            
            Mood::Grumpy => MoodKeywords {
                variable_declaration: "ugh",
                function_declaration: "do_this_for_me",
                conditional: "whatever",
                loop_construct: "repeat_i_guess",
                return_statement: "take_it",
                end_block: "finally",
                assignment: "equals",
                comment_marker: "ignore_this",
            },
            
            Mood::Philosophical => MoodKeywords {
                variable_declaration: "ponder",
                function_declaration: "essence_of",
                conditional: "what_if",
                loop_construct: "eternal_return",
                return_statement: "conclude",
                end_block: "fin",
                assignment: "becomes",
                comment_marker: "contemplate",
            },
            
            Mood::Sarcastic => MoodKeywords {
                variable_declaration: "supposedly",
                function_declaration: "try_doing",
                conditional: "as_if",
                loop_construct: "round_and_round",
                return_statement: "there_ya_go",
                end_block: "whoopee",
                assignment: "totally_equals",
                comment_marker: "yeah_right",
            },
            
            Mood::Confused => MoodKeywords {
                variable_declaration: "umm",
                function_declaration: "somehow",
                conditional: "maybe",
                loop_construct: "do_more",
                return_statement: "is_this_right",
                end_block: "i_think_im_done",
                assignment: "could_be",
                comment_marker: "what",
            },
        }
    }
    
    /// Get the error message style for the current mood
    pub fn get_error_style(&self) -> ErrorStyle {
        match self {
            Mood::Happy => ErrorStyle::Encouraging,
            Mood::Grumpy => ErrorStyle::Harsh, 
            Mood::Philosophical => ErrorStyle::Existential,
            Mood::Sarcastic => ErrorStyle::Mocking,
            Mood::Confused => ErrorStyle::Uncertain,
        }
    }
    
    /// Get operators based on the current mood
    pub fn get_operators(&self) -> MoodOperators {
        match self {
            Mood::Happy => MoodOperators {
                addition: "plus",
                subtraction: "minus",
                multiplication: "times",
                division: "divided_by",
                equality: "same_as",
                inequality: "different_from",
            },
            
            Mood::Grumpy => MoodOperators {
                addition: "add",
                subtraction: "subtract",
                multiplication: "multiply",
                division: "divide",
                equality: "equals",
                inequality: "not_equals",
            },
            
            Mood::Philosophical => MoodOperators {
                addition: "combine",
                subtraction: "reduce",
                multiplication: "amplify",
                division: "diminish",
                equality: "identical",
                inequality: "distinct",
            },
            
            Mood::Sarcastic => MoodOperators {
                addition: "throw_in",
                subtraction: "take_away",
                multiplication: "duplicate",
                division: "share",
                equality: "sure_same",
                inequality: "obviously_different",
            },
            
            Mood::Confused => MoodOperators {
                addition: "more",
                subtraction: "less",
                multiplication: "lots",
                division: "split",
                equality: "is_it_same",
                inequality: "is_it_different",
            },
        }
    }
}

pub struct MoodKeywords {
    pub variable_declaration: &'static str,
    pub function_declaration: &'static str,
    pub conditional: &'static str,
    pub loop_construct: &'static str,
    pub return_statement: &'static str,
    pub end_block: &'static str, 
    pub assignment: &'static str,
    pub comment_marker: &'static str,
}

pub struct MoodOperators {
    pub addition: &'static str,
    pub subtraction: &'static str,
    pub multiplication: &'static str,
    pub division: &'static str,
    pub equality: &'static str,
    pub inequality: &'static str,
}

pub enum ErrorStyle {
    Encouraging, // "Don't worry! You just forgot a semicolon. You're doing great!"
    Harsh,       // "WRONG. You made an obvious mistake on line 42."
    Existential, // "Is a missing bracket truly an error, or merely a different path?"
    Mocking,     // "Oh sure, like that code was EVER going to work..."
    Uncertain,   // "Um, I think maybe there's something wrong with line 10? Or not?"
} 