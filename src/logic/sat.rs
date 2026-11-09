//! Exercice 07 : satisfiabilité (SAT).
//!
//! Une formule est satisfiable s'il existe au moins une affectation des variables
//! qui la rend vraie. On réutilise l'énumération des lignes de la table de vérité
//! et on s'arrête à la première ligne vraie. Complexité : O(2^n) au pire.

use super::error::FormulaError;
use super::parser::parse_rpn;
use super::truth_table::{evaluate_row, row_count};

/// Version qui retourne l'erreur au lieu de l'afficher.
pub fn try_sat(formula: &str) -> Result<bool, FormulaError> {
    let tree = parse_rpn(formula)?;
    let variables = tree.variables();

    for row_index in 0..row_count(variables.len()) {
        if evaluate_row(&tree, &variables, row_index)? {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Prototype imposé par le sujet. Formule invalide : message sur stderr, puis `false`.
pub fn sat(formula: &str) -> bool {
    try_sat(formula).unwrap_or_else(|error| {
        eprintln!("Erreur : {error}");
        false
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_subject_examples() {
        assert!(sat("AB|"));
        assert!(sat("AB&"));
        assert!(!sat("AA!&"));
        assert!(!sat("AA^"));
    }

    #[test]
    fn tautologies_and_contradictions() {
        assert!(sat("AA!|")); // A ∨ ¬A
        assert!(!sat("AA!&")); // A ∧ ¬A
        assert!(!sat("AB^AB=&")); // (A⊕B) ∧ (A⇔B)
    }

    #[test]
    fn constants() {
        assert!(sat("1"));
        assert!(!sat("0"));
        assert!(!sat("1!"));
    }

    #[test]
    fn satisfiable_only_by_one_specific_row() {
        assert!(sat("AB&C&D&")); // seul 1111 convient
        assert!(sat("A!B!&C!&")); // seul 000 convient
    }

    #[test]
    fn invalid_formula_returns_false_and_an_error() {
        assert_eq!(try_sat("&"), Err(FormulaError::MissingOperand('&')));
        assert!(!sat("AB"));
    }
}
