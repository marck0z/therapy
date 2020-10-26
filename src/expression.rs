use std::error::Error;

use evalexpr::*;

use substitution::*;

use crate::{Input, Output};

mod substitution;

/// Holds two vectors containing H substitutions and K substitutions
#[derive(Clone)]
pub(crate) struct ExpressionProcessor {
    h_substitutions: Vec<HSubstitution>,
    k_substitutions: Vec<KSubstitution>,
}

/// Process the string representing an expression an creates a H or K substitution
impl ExpressionProcessor {
    /// Creates a new ExpressionProcessor from two vectors of expressions in string form
    /// it parses each string an creates the corresponding substitution
    ///
    /// # Arguments
    ///
    /// * `h_subs_str` - A collection of H substitutions
    /// * `k_subs_str` - A collection of K substitutions
    pub(crate) fn new(h_subs_str: Vec<&str>, k_subs_str: Vec<&str>) -> ExpressionProcessor {
        // Parse the H substitutions
        let mut h_subs = Vec::new();
        for h in h_subs_str {
            let h_sub = HSubstitution::new(h);
            match h_sub {
                Ok(h_sub) => h_subs.push(h_sub),
                Err(_) => println!("invalid H Substitution: {}", h)
            }
        }

        // Parse the K substitutions
        let mut k_subs = Vec::new();
        for k in k_subs_str {
            let k_sub = KSubstitution::new(k);
            match k_sub {
                Ok(k_sub) => k_subs.push(k_sub),
                Err(_) => println!("invalid K Substitution: {}", k)
            }
        }

        ExpressionProcessor {
            h_substitutions: h_subs,
            k_substitutions: k_subs,
        }
    }

    /// Evaluates an input against the substitutions in reverse order to account for overriding
    /// if a coincidence is found it returns immediately
    ///
    /// # Arguments
    ///
    /// * `input` - An Input Struct containing the value of the variables A to F
    pub(crate) fn evaluate(&self, input: &Input) -> Result<Output, Box<dyn Error>> {
        let mut context: HashMapContext = context_map! {
            "A" => input.a,
            "B" => input.b,
            "C" => input.c,
            "D" => input.d,
            "E" => input.e,
            "F" => input.f
        }?;

        //evaluate H value
        let h = self.h_substitutions.iter().rev().find_map(|h| h.evaluate(&context));
        let h = match h {
            Some(v) => v,
            None => { return Err("H not found".into()); }
        };

        //evaluate K value
        let k = self.k_substitutions.iter().rev().find_map(|k| k.evaluate(h, &mut context));
        let k = match k {
            Some(v) => v,
            None => { return Err("K not found".into()); }
        };

        Ok(Output {
            h,
            k,
        })
    }
}

#[cfg(test)]
mod tests;