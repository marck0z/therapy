use std::error::Error;

use evalexpr::*;

///Represents a new H value if the input pattern matches
#[derive(Debug, PartialEq, Clone)]
pub(super) struct HSubstitution {
    input: String,
    output: char,
}

impl HSubstitution {
    /// Parse a H substitution expression.
    /// Because of the scope of this assignment, it's assumed that
    /// the substitution expression provided is valid
    ///
    /// # Arguments
    ///
    /// * `exp` - A string representing a 'H' Substitution if true
    pub(super) fn new(exp: &str) -> Result<HSubstitution, Box<dyn Error>> {
        let tokens: Vec<&str> = exp.split("=>").collect();

        //check if the expression has 2 sides
        if tokens.len() != 2 {
            Err("invalid expression".into())
        } else {
            //parse the life side of the expression
            let left = tokens[0].trim().to_owned();

            //parse the right side of the expression
            let right = tokens[1].split('=').last().and_then(|s| s.trim().chars().next());

            match right {
                Some(right) => Ok(HSubstitution {
                    input: left,
                    output: right,
                }),
                None => Err("invalid expression".into())
            }
        }
    }

    /// Calculate the value of H
    /// If the current Substitution evaluates to True then return the current output
    /// # Arguments
    ///
    /// * `context` - The values of variables A to C
    pub(super) fn evaluate(&self, context: &HashMapContext) -> Option<char> {
        println!("evaluating: {}", self.input);
        match eval_boolean_with_context(&self.input, context) {
            Ok(b) => if b { Some(self.output) } else { None },
            Err(e) => {
                println!("couldn't evaluate: {} error: {}", self.input, e);
                None
            }
        }
    }
}

///Represents a new K value calculated from a pattern if the input matches
#[derive(Debug, PartialEq, Clone)]
pub(super) struct KSubstitution {
    input: char,
    output: String,
}

impl KSubstitution {
    /// Parse a K substitution expression.
    /// Because of the scope of this assignment, it's assumed that
    /// the substitution expression provided is valid
    ///
    /// # Arguments
    ///
    /// * `exp` - A string representing a K Substitution if true
    pub(super) fn new(exp: &str) -> Result<KSubstitution, Box<dyn Error>> {
        let tokens: Vec<&str> = exp.split("=>").collect();

        //check if the expression has 2 sides
        if tokens.len() != 2 {
            Err("invalid expression".into())
        } else {
            //parse the right side of the expression
            let right = tokens[1].trim().to_owned();

            //parse the left side of the expression
            let left = tokens[0].split('=').last().and_then(|s| s.trim().chars().next());

            match left {
                Some(left) => Ok(KSubstitution {
                    input: left,
                    output: right,
                }),
                None => Err("invalid expression".into())
            }
        }
    }

    /// Calculate the value of K
    /// # Arguments
    ///
    /// * `h` - The value of the H variable
    /// * `context` - The values of variables D, E, F
    pub(super) fn evaluate(&self, h: char, context: &mut HashMapContext) -> Option<f64> {
        if self.input == h {
            println!("evaluating: {}", self.output);
            let _ = eval_empty_with_context_mut(&self.output, context);
            context.get_value("K").and_then(|v| v.as_float().ok())
        } else { None }
    }
}

#[cfg(test)]
mod tests;