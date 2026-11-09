//! Ready, Set, Boole! : introduction à l'algèbre de Boole.
//!
//! Les fonctions du sujet sont re-exportées ici avec leurs prototypes d'origine.

pub mod arithmetic;
pub mod curve;
pub mod logic;
pub mod sets;

pub use arithmetic::{adder, gray_code, multiplier};
pub use curve::{map, reverse_map};
pub use logic::{
    conjunctive_normal_form, eval_formula, negation_normal_form, print_truth_table, sat,
};
pub use sets::{eval_set, powerset};
