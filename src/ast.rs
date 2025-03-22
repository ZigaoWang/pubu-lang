use std::collections::HashMap;

/// AST for the PUBU language
#[derive(Debug, Clone)]
pub enum Node {
    Program(Vec<Node>),
    
    // Variables and expressions
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    VariableDeclaration(String, Box<Node>),
    
    // Control flow
    IfStatement(Box<Node>, Vec<Node>, Option<Vec<Node>>),
    Loop(Vec<Node>),
    MoodBlock(MoodType, Vec<Node>),
    LunarPhaseBlock(Vec<Node>),
    
    // Functions
    FunctionDeclaration(String, Vec<String>, Vec<Node>),
    FunctionCall(String, Vec<Node>),
    Return(Option<Box<Node>>),
    
    // Unique PUBU constructs
    Compliment(String),
    PhilosophicalQuestion(String),
    RandomFailure,
    RealityDistortion(Box<Node>),
    
    // Base operations
    BinaryOp(Box<Node>, Operator, Box<Node>),
    UnaryOp(Operator, Box<Node>),
    Assignment(String, Box<Node>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    // Standard operators
    Add,
    Subtract,
    Multiply,
    Divide,
    
    // Comparison
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    
    // Logical
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoodType {
    Happy,
    Grumpy,
    Philosophical,
    Sarcastic,
    Confused,
}

/// Runtime environment for executing PUBU code
pub struct Environment {
    pub variables: HashMap<String, Value>,
    pub functions: HashMap<String, Function>,
    pub parent: Option<Box<Environment>>,
    pub compliment_count: usize,
    pub philosophical_question_count: usize,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
            compliment_count: 0,
            philosophical_question_count: 0,
        }
    }
    
    pub fn with_parent(parent: Environment) -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: Some(Box::new(parent)),
            compliment_count: 0,
            philosophical_question_count: 0,
        }
    }
    
    pub fn get(&self, name: &str) -> Option<Value> {
        match self.variables.get(name) {
            Some(value) => Some(value.clone()),
            None => {
                match &self.parent {
                    Some(parent) => parent.get(name),
                    None => None,
                }
            }
        }
    }
    
    pub fn set(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }
    
    pub fn define_function(&mut self, name: &str, params: Vec<String>, body: Vec<Node>) {
        self.functions.insert(name.to_string(), Function { params, body });
    }
    
    pub fn get_function(&self, name: &str) -> Option<Function> {
        match self.functions.get(name) {
            Some(function) => Some(function.clone()),
            None => {
                match &self.parent {
                    Some(parent) => parent.get_function(name),
                    None => None,
                }
            }
        }
    }
    
    pub fn add_compliment(&mut self) {
        self.compliment_count += 1;
    }
    
    pub fn add_philosophical_question(&mut self) {
        self.philosophical_question_count += 1;
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<Node>,
}

/// Values in the PUBU language
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Function(Function),
    Null,
    
    // PUBU unique types
    Uncertain(Box<Value>, f64), // Value with a confidence level (0.0-1.0)
    Schrodinger(Box<Value>, Box<Value>), // Value that's in two states at once until observed
    Whimsical(String), // Value that defies definition but pretends to have meaning
}

impl Value {
    /// Apply reality distortion to the value
    pub fn distort(&self) -> Value {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        match self {
            Value::Number(n) => {
                let distortion = rng.gen_range(-0.5..0.5);
                Value::Number(n + n * distortion)
            },
            Value::String(s) => {
                if rng.gen_bool(0.3) {
                    let reversed: String = s.chars().rev().collect();
                    Value::String(reversed)
                } else {
                    Value::String(s.clone())
                }
            },
            Value::Boolean(b) => {
                if rng.gen_bool(0.2) {
                    Value::Boolean(!b)
                } else {
                    Value::Boolean(*b)
                }
            },
            Value::Uncertain(v, confidence) => {
                let new_confidence = confidence * rng.gen_range(0.5..1.5);
                Value::Uncertain(v.clone(), new_confidence.min(1.0).max(0.0))
            },
            Value::Schrodinger(v1, v2) => {
                if rng.gen_bool(0.5) {
                    Value::Schrodinger(v2.clone(), v1.clone())
                } else {
                    Value::Schrodinger(v1.clone(), v2.clone())
                }
            },
            _ => self.clone(),
        }
    }
} 