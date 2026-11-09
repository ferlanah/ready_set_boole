//! Exercices 03 à 07 : logique propositionnelle.

mod cnf;
mod error;
mod evaluator;
mod formula;
mod nnf;
mod parser;
mod printer;
mod sat;
#[cfg(test)]
mod test_support;
mod truth_table;

pub use cnf::{conjunctive_normal_form, try_conjunctive_normal_form};
pub use error::FormulaError;
pub use evaluator::{eval_formula, evaluate, try_eval_formula, Assignment};
pub use formula::Formula;
pub use nnf::{negation_normal_form, try_negation_normal_form};
pub use parser::parse_rpn;
pub use printer::to_rpn;
pub use sat::{sat, try_sat};
pub use truth_table::{print_truth_table, render_truth_table};
