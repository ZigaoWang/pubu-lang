use crate::ast::{Node, Operator, Value, Environment, MoodType};
use crate::error::PubuError;
use crate::ai_critic::AiCritic;
use rand::Rng;
use colored::*;

pub struct Interpreter {
    env: Environment,
    ai_critic: AiCritic,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            ai_critic: AiCritic::new(),
        }
    }
    
    pub fn interpret(&mut self, program: Node) -> Result<Value, PubuError> {
        // Random chance of failure regardless of program correctness
        if rand::thread_rng().gen_bool(0.05) {  // 5% chance
            return Err(PubuError::RandomFailure);
        }
        
        self.evaluate(program)
    }
    
    fn evaluate(&mut self, node: Node) -> Result<Value, PubuError> {
        match node {
            Node::Program(statements) => {
                let mut result = Value::Null;
                
                // Check if we have enough compliments
                let compliment_count = statements.iter()
                    .filter(|s| matches!(s, Node::Compliment(_)))
                    .count();
                
                if compliment_count < 1 {
                    return Err(PubuError::NotEnoughCompliments);
                }
                
                // Check if there are philosophical questions in functions
                let has_functions = statements.iter()
                    .any(|s| matches!(s, Node::FunctionDeclaration(_, _, _)));
                
                let has_philosophical_questions = statements.iter()
                    .any(|s| matches!(s, Node::PhilosophicalQuestion(_)));
                
                if has_functions && !has_philosophical_questions {
                    return Err(PubuError::PhilosophicalCrisis);
                }
                
                // Process all statements
                for statement in statements {
                    result = self.evaluate(statement)?;
                    
                    // Sometimes distort reality between statements
                    if rand::thread_rng().gen_bool(0.1) {  // 10% chance
                        result = result.distort();
                    }
                }
                
                Ok(result)
            },
            
            Node::Compliment(text) => {
                // Process the compliment
                self.env.add_compliment();
                println!("{}", format!("PUBU appreciates your compliment: '{}'", text).green());
                Ok(Value::String(text))
            },
            
            Node::PhilosophicalQuestion(question) => {
                // Process the philosophical question
                self.env.add_philosophical_question();
                println!("{}", format!("PUBU ponders: '{}'", question).blue().italic());
                Ok(Value::Whimsical(question))
            },
            
            Node::Number(n) => Ok(Value::Number(n)),
            Node::String(s) => Ok(Value::String(s)),
            Node::Boolean(b) => Ok(Value::Boolean(b)),
            
            Node::LunarPhaseBlock(statements) => {
                // Check if the current lunar phase allows execution
                if !crate::is_lunar_compatible() {
                    return Err(PubuError::MoonPhaseIncompatible);
                }
                
                // Execute the block's statements
                let mut result = Value::Null;
                for statement in statements {
                    result = self.evaluate(statement)?;
                }
                
                Ok(result)
            },
            
            Node::MoodBlock(mood_type, statements) => {
                // Process blocks with mood-specific rules
                match mood_type {
                    MoodType::Grumpy => {
                        // Grumpy blocks have a chance to fail
                        if rand::thread_rng().gen_bool(0.3) {  // 30% chance
                            return Err(PubuError::BadMood);
                        }
                    },
                    MoodType::Philosophical => {
                        // Philosophical blocks require a philosophical question
                        let has_philosophical_question = statements.iter()
                            .any(|s| matches!(s, Node::PhilosophicalQuestion(_)));
                        
                        if !has_philosophical_question {
                            return Err(PubuError::PhilosophicalCrisis);
                        }
                    },
                    _ => {}  // Other moods don't have special rules
                }
                
                // Execute the block's statements
                let mut result = Value::Null;
                for statement in statements {
                    result = self.evaluate(statement)?;
                }
                
                Ok(result)
            },
            
            Node::RealityDistortion(expr) => {
                // Evaluate the expression and then distort its value
                let value = self.evaluate(*expr)?;
                Ok(value.distort())
            },
            
            Node::RandomFailure => {
                // This node always results in failure
                Err(PubuError::RandomFailure)
            },
            
            // For simplicity, all other operations return placeholder values
            // In a real implementation, they would be fully implemented
            _ => {
                // Simulating a basic execution for the placeholder
                println!("{}", "PUBU is pretending to execute something...".yellow());
                
                // Occasionally critique the code for no reason
                if rand::thread_rng().gen_bool(0.2) {  // 20% chance
                    let code_critique = self.ai_critic.critique("placeholder code");
                    println!("\n{}\n", code_critique);
                }
                
                Ok(Value::Null)
            }
        }
    }
} 