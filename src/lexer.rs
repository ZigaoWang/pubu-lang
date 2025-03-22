use crate::mood::Mood;
use crate::error::PubuError;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Basic tokens
    Identifier(String),
    Number(f64),
    String(String),
    
    // Keywords based on mood
    VariableDeclaration, // like "yay", "ugh", "ponder" depending on mood
    FunctionDeclaration, // like "celebration", "do_this_for_me", etc.
    Conditional,         // like "perhaps", "whatever", etc.
    Loop,                // like "again_and_again", "repeat_i_guess", etc.
    Return,              // like "here_you_go", "take_it", etc.
    EndBlock,            // like "done", "finally", etc.
    
    // Operators (also mood-dependent)
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    
    // Special PUBU constructs
    Compliment,
    PhilosophicalQuestion,
    MoodDeclaration,
    LunarPhase,
    
    // Syntactic elements
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Semicolon,
    Comma,
    Assignment,
    
    // Misc
    Comment,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self { token_type, lexeme, line }
    }
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    current_mood: Mood,
}

impl Lexer {
    pub fn new(source: String, mood: Mood) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            current_mood: mood,
        }
    }
    
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, PubuError> {
        // Randomly refuse to tokenize for no reason
        if rand::thread_rng().gen_bool(0.05) {  // 5% chance
            return Err(PubuError::RandomFailure);
        }
        
        // The general lexing would happen here
        // This is a placeholder implementation
        
        // Add a placeholder compliment token - this would be actual
        // tokenization logic in a real implementation
        self.tokens.push(Token::new(
            TokenType::Compliment,
            "dear_pubu_you_look_gorgeous_today".to_string(),
            1
        ));
        
        // Add EOF token
        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            self.line
        ));
        
        Ok(self.tokens.clone())
    }
    
    // Helper functions for lexing would go here
    
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        c
    }
    
    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, text.to_string(), self.line));
    }
    
    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source.chars().nth(self.current) != Some(expected) { return false; }
        
        self.current += 1;
        true
    }
    
    // Detect mood-specific keywords
    fn identifier_type(&self, text: &str) -> TokenType {
        match self.current_mood {
            Mood::Happy => {
                match text {
                    "yay" => TokenType::VariableDeclaration,
                    "celebration" => TokenType::FunctionDeclaration,
                    "perhaps" => TokenType::Conditional,
                    "again_and_again" => TokenType::Loop,
                    "here_you_go" => TokenType::Return,
                    "done" => TokenType::EndBlock,
                    "happy" => TokenType::MoodDeclaration,
                    "dear_pubu" | "gorgeous" | "brilliant" | "amazing" => TokenType::Compliment,
                    _ => TokenType::Identifier(text.to_string()),
                }
            },
            Mood::Grumpy => {
                match text {
                    "ugh" => TokenType::VariableDeclaration,
                    "do_this_for_me" => TokenType::FunctionDeclaration,
                    "whatever" => TokenType::Conditional,
                    "repeat_i_guess" => TokenType::Loop,
                    "take_it" => TokenType::Return,
                    "finally" => TokenType::EndBlock,
                    "grumpy" => TokenType::MoodDeclaration,
                    "fine_pubu" | "not_bad" | "decent" => TokenType::Compliment,
                    _ => TokenType::Identifier(text.to_string()),
                }
            },
            // Additional moods would be handled similarly
            _ => TokenType::Identifier(text.to_string()), // Simplified
        }
    }
    
    // Detect operators based on mood
    fn operator_type(&self, text: &str) -> Option<TokenType> {
        match self.current_mood {
            Mood::Happy => {
                match text {
                    "plus" => Some(TokenType::Plus),
                    "minus" => Some(TokenType::Minus),
                    "times" => Some(TokenType::Times),
                    "divided_by" => Some(TokenType::Divide),
                    "same_as" => Some(TokenType::Equal),
                    "different_from" => Some(TokenType::NotEqual),
                    _ => None,
                }
            },
            Mood::Grumpy => {
                match text {
                    "add" => Some(TokenType::Plus),
                    "subtract" => Some(TokenType::Minus),
                    "multiply" => Some(TokenType::Times),
                    "divide" => Some(TokenType::Divide),
                    "equals" => Some(TokenType::Equal),
                    "not_equals" => Some(TokenType::NotEqual),
                    _ => None,
                }
            },
            // Additional moods would be handled similarly
            _ => None, // Simplified
        }
    }
    
    // Detect philosophical questions
    fn is_philosophical_question(&self, text: &str) -> bool {
        text.ends_with("?") && 
        (text.contains("why") || 
         text.contains("meaning") || 
         text.contains("purpose") || 
         text.contains("existence"))
    }
} 