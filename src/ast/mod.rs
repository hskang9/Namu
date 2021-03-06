use crate::token::*;
use std::clone::Clone;
use std::any::Any;


//TODO: Leave it until Rust supports multiple inheritance for Box<{any struct with interface}>
/*
pub trait Node {
    fn token_literal(&self) -> String;
}
*/

/// Generic Statements (e.g. let a = b) interface
pub trait Statement {
    fn token_literal(&self) -> String;
    fn statement_node(&self);
    fn string(&self) -> String;
    fn box_clone(&self) -> Box<dyn Statement>;
    fn type_name(&self) -> String;
    fn to_any(&self) -> Box<dyn Any + 'static>;
}

/// Clone for trait object 
impl Clone for Box<dyn Statement> {
    fn clone(&self) -> Box<dyn Statement> {
        self.box_clone()
    }
}

/// Expressions (e.g. {}, ()) interface
pub trait Expression {
    fn token_literal(&self) -> String;
    fn expression_node(&self);
    fn string(&self) -> String;
    fn box_clone(&self) -> Box<dyn Expression>;   
    fn type_name(&self) -> String;
    fn to_any(&self) -> Box<dyn Any + 'static>;
}


/// Clone for trait object 
impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Box<dyn Expression> {
        self.box_clone()
    }
}

/// Program after reading the code
#[derive(Clone)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Statement for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return "".to_string();
        }
    }
    fn statement_node(&self) {}
    fn string(&self) -> String {
        let mut out = "".to_string();
        for s in self.statements.clone() {
            out.push_str(&s.token_literal());
        }
        return out.to_string();
    }
    fn box_clone(&self) -> Box<dyn Statement> {
        Box::new((*self).clone())
    }
    fn type_name(&self) -> String {
        return type_of(&self);
    }
    fn to_any(&self) -> Box<dyn Any + 'static> {
       Box::new((*self).clone())
    }
}


/// Let Statment
#[derive(Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Option<Identifier>,
    pub value: Option<Box<dyn Expression>>,
}

impl Statement for LetStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.to_string();
    }
    fn statement_node(&self) {}
    fn string(&self) -> String {
        let out = &[self.token_literal(), " ".to_string(), self.name.clone().unwrap().value, " = ".to_string()].concat();

        if self.value.is_some() {
            let out2 = [out.clone(), self.value.clone().unwrap().string()].concat();
            out2.to_string();
        }
        return out.to_string();
    }
    fn box_clone(&self) -> Box<dyn Statement> {
        Box::new((*self).clone())
    }
    fn type_name(&self) -> String {
        return type_of(&self);
    }
    fn to_any(&self) -> Box<dyn Any + 'static> {
        Box::new((*self).clone())
     }
}


/// Identifier, your variable name
#[derive(Clone)]
pub struct Identifier {
    pub token: Option<Token>,
    pub value: String,
}

impl Expression for Identifier {
    fn token_literal(&self) -> String {
        return self.token.clone().unwrap().literal;
    }
    fn expression_node(&self) {}
    fn string(&self) -> String {
        return self.value.clone();
    }
    fn box_clone(&self) -> Box<dyn Expression> {
        Box::new((*self).clone())
    }
    fn type_name(&self) -> String {
        return type_of(&self);
    }
    fn to_any(&self) -> Box<dyn Any + 'static> {
        Box::new((*self).clone())
     }
}

// Return statement
#[derive(Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}

impl Statement for ReturnStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.to_string();
    }
    fn statement_node(&self) {}
    fn string(&self) -> String {
        let out = &[&self.token_literal(), " "].concat();
        
        if self.return_value.is_some() {
            let out2 = &[out.clone(), self.return_value.clone().unwrap().string()].concat();
            return out2.to_string();
        }
        
        return out.to_string();
    }
    fn box_clone(&self) -> Box<dyn Statement> {
        Box::new((*self).clone())
    }
    fn type_name(&self) -> String {
        return type_of(&self);
    }
    fn to_any(&self) -> Box<dyn Any + 'static> {
        Box::new((*self).clone())
     }
}



// Expression statement
#[derive(Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl Statement for ExpressionStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.to_string();
    }
    fn statement_node(&self) {}
    fn string(&self) -> String  {
        if self.expression.is_some() {
            return self.expression.clone().unwrap().string();
        }
        return "".to_string();
    }
    fn box_clone(&self) -> Box<dyn Statement> {
        Box::new((*self).clone())
    }
    fn type_name(&self) -> String {
        return type_of(&self);
    }
    fn to_any(&self) -> Box<dyn Any + 'static> {
        Box::new((*self).clone())
     }
}



/// Integer literal
#[derive(Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: Option<i64>
}

impl Expression for IntegerLiteral {
    fn token_literal(&self) -> String {
        return self.token.clone().literal;
    }
    fn expression_node(&self) {}
    fn string(&self) -> String {
        return self.token.clone().literal;
    }
    fn box_clone(&self) -> Box<dyn Expression> {
        Box::new((*self).clone())
    }
    fn type_name(&self) -> String {
        return type_of(&self);
    }
    fn to_any(&self) -> Box<dyn Any + 'static> {
        Box::new((*self).clone())
    }
}

pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
