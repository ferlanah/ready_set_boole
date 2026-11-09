//! Exercice 05 : forme normale négative (NNF).
//!
//! Objectif : n'avoir que `!`, `&`, `|`, et chaque `!` juste devant une variable.
//! Deux familles de règles du sujet (Interlude 00) :
//! 1. supprimer `^`, `>`, `=` en les réécrivant avec `!`, `&`, `|` ;
//! 2. pousser les `!` vers les feuilles (double négation et lois de De Morgan).
//!
//! `push_negations_down` fait les deux en un seul parcours : il porte un booléen
//! `negated` qui signifie « un nombre impair de `!` se trouve au-dessus de ce nœud ».

use super::error::FormulaError;
use super::formula::Formula;
use super::parser::parse_rpn;
use super::printer::to_rpn;

/// Convertit l'arbre en NNF (utilisé aussi par la CNF).
pub fn to_nnf(formula: &Formula) -> Formula {
    push_negations_down(formula, false)
}

/// Descend les négations vers les feuilles.
///
/// - `negated == false` : on construit la NNF de `formula`.
/// - `negated == true`  : on construit la NNF de `¬formula`.
fn push_negations_down(formula: &Formula, negated: bool) -> Formula {
    match formula {
        Formula::Const(value) => Formula::Const(*value != negated),
        Formula::Var(name) => literal(*name, negated),
        // ¬¬A ⇔ A : une négation de plus inverse simplement le drapeau.
        Formula::Not(operand) => push_negations_down(operand, !negated),
        Formula::And(left, right) => push_into_and(left, right, negated),
        Formula::Or(left, right) => push_into_or(left, right, negated),
        Formula::Xor(left, right) => {
            push_negations_down(&rewrite_xor(left, right), negated)
        }
        Formula::Implies(left, right) => {
            push_negations_down(&rewrite_implication(left, right), negated)
        }
        Formula::Equiv(left, right) => {
            push_negations_down(&rewrite_equivalence(left, right), negated)
        }
    }
}

/// Une variable, précédée d'un `!` si `negated`.
fn literal(name: char, negated: bool) -> Formula {
    let variable = Formula::Var(name);
    if negated {
        Formula::Not(Box::new(variable))
    } else {
        variable
    }
}

/// `A ∧ B`, ou `¬(A ∧ B) ⇔ ¬A ∨ ¬B` (De Morgan) si `negated`.
fn push_into_and(left: &Formula, right: &Formula, negated: bool) -> Formula {
    let new_left = Box::new(push_negations_down(left, negated));
    let new_right = Box::new(push_negations_down(right, negated));
    if negated {
        Formula::Or(new_left, new_right)
    } else {
        Formula::And(new_left, new_right)
    }
}

/// `A ∨ B`, ou `¬(A ∨ B) ⇔ ¬A ∧ ¬B` (De Morgan) si `negated`.
fn push_into_or(left: &Formula, right: &Formula, negated: bool) -> Formula {
    let new_left = Box::new(push_negations_down(left, negated));
    let new_right = Box::new(push_negations_down(right, negated));
    if negated {
        Formula::And(new_left, new_right)
    } else {
        Formula::Or(new_left, new_right)
    }
}

/// `A ⇒ B` devient `¬A ∨ B` (condition matérielle).
fn rewrite_implication(left: &Formula, right: &Formula) -> Formula {
    let not_left = Formula::Not(Box::new(left.clone()));
    Formula::Or(Box::new(not_left), Box::new(right.clone()))
}

/// `A ⇔ B` devient `(A ∧ B) ∨ (¬A ∧ ¬B)` : « les deux vrais ou les deux faux ».
fn rewrite_equivalence(left: &Formula, right: &Formula) -> Formula {
    let both_true = Formula::And(Box::new(left.clone()), Box::new(right.clone()));
    let both_false = Formula::And(
        Box::new(Formula::Not(Box::new(left.clone()))),
        Box::new(Formula::Not(Box::new(right.clone()))),
    );
    Formula::Or(Box::new(both_true), Box::new(both_false))
}

/// `A ⊕ B` devient `(A ∧ ¬B) ∨ (¬A ∧ B)` : « exactement un des deux est vrai ».
fn rewrite_xor(left: &Formula, right: &Formula) -> Formula {
    let only_left = Formula::And(
        Box::new(left.clone()),
        Box::new(Formula::Not(Box::new(right.clone()))),
    );
    let only_right = Formula::And(
        Box::new(Formula::Not(Box::new(left.clone()))),
        Box::new(right.clone()),
    );
    Formula::Or(Box::new(only_left), Box::new(only_right))
}

/// Version qui retourne l'erreur au lieu de l'afficher.
pub fn try_negation_normal_form(formula: &str) -> Result<String, FormulaError> {
    let tree = parse_rpn(formula)?;
    Ok(to_rpn(&to_nnf(&tree)))
}

/// Prototype imposé par le sujet. Formule invalide : message sur stderr, puis chaîne vide.
pub fn negation_normal_form(formula: &str) -> String {
    try_negation_normal_form(formula).unwrap_or_else(|error| {
        eprintln!("Erreur : {error}");
        String::new()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::test_support::{are_equivalent, is_nnf};

    #[test]
    fn matches_subject_examples() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
    }

    #[test]
    fn double_negation_is_removed() {
        assert_eq!(negation_normal_form("A!!"), "A");
        assert_eq!(negation_normal_form("A!!!"), "A!");
    }

    #[test]
    fn already_normal_formulas_are_unchanged() {
        assert_eq!(negation_normal_form("A"), "A");
        assert_eq!(negation_normal_form("A!B|"), "A!B|");
    }

    #[test]
    fn result_is_nnf_and_equivalent_for_every_operator_and_nesting() {
        let formulas = [
            "AB^", "AB^!", "AB>!", "AB=!", "AB&!C|!", "AB>C=!", "AB^C>D=!", "A!B!^!",
            "AB|CD&^!", "ABC>=!",
        ];
        for rpn in formulas {
            let original = parse_rpn(rpn).unwrap();
            let normalized = to_nnf(&original);
            assert!(is_nnf(&normalized), "{rpn} -> {}", to_rpn(&normalized));
            assert!(are_equivalent(&original, &normalized), "{rpn}");
        }
    }

    #[test]
    fn output_only_contains_variables_and_allowed_symbols() {
        let output = negation_normal_form("AB^C>D=!");
        assert!(output.chars().all(|c| c.is_ascii_uppercase() || "!&|".contains(c)));
    }

    #[test]
    fn invalid_formula_gives_empty_string() {
        assert_eq!(try_negation_normal_form("&"), Err(FormulaError::MissingOperand('&')));
        assert_eq!(negation_normal_form("AB"), "");
    }
}
