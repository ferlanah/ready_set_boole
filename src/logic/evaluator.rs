//! Exercice 03 : évaluation d'une formule booléenne.
//!
//! Chaîne de calcul : `&str` --parse_rpn--> `Formula` --evaluate--> `bool`.
//! Complexité : le parsing et l'évaluation visitent chaque nœud une fois, soit O(n).

use super::error::FormulaError;
use super::formula::Formula;
use super::parser::parse_rpn;
use std::collections::BTreeMap;

/// Valeur de vérité de chaque variable (ex. `{'A': true, 'B': false}`).
pub type Assignment = BTreeMap<char, bool>;

/// Évalue l'arbre sous l'affectation donnée.
///
/// Récursion : la valeur d'un nœud est calculée à partir de celles de ses enfants.
pub fn evaluate(formula: &Formula, assignment: &Assignment) -> Result<bool, FormulaError> {
    match formula {
        Formula::Const(value) => Ok(*value),
        Formula::Var(name) => lookup_variable(*name, assignment),
        Formula::Not(operand) => Ok(!evaluate(operand, assignment)?),
        Formula::And(l, r) => combine(l, r, assignment, and),
        Formula::Or(l, r) => combine(l, r, assignment, or),
        Formula::Xor(l, r) => combine(l, r, assignment, xor),
        Formula::Implies(l, r) => combine(l, r, assignment, implies),
        Formula::Equiv(l, r) => combine(l, r, assignment, equivalent),
    }
}

/// Cherche la valeur d'une variable, ou signale qu'elle n'en a pas.
fn lookup_variable(name: char, assignment: &Assignment) -> Result<bool, FormulaError> {
    assignment
        .get(&name)
        .copied()
        .ok_or(FormulaError::UndefinedVariable(name))
}

/// Évalue les deux enfants puis applique l'opérateur `operator` à leurs valeurs.
fn combine(
    left: &Formula,
    right: &Formula,
    assignment: &Assignment,
    operator: fn(bool, bool) -> bool,
) -> Result<bool, FormulaError> {
    let left_value = evaluate(left, assignment)?;
    let right_value = evaluate(right, assignment)?;
    Ok(operator(left_value, right_value))
}

// Les 5 opérateurs binaires : chacun est sa propre table de vérité.

fn and(left: bool, right: bool) -> bool {
    left && right
}

fn or(left: bool, right: bool) -> bool {
    left || right
}

/// Vrai si exactement un des deux est vrai (⊕).
fn xor(left: bool, right: bool) -> bool {
    left != right
}

/// Condition matérielle A ⇒ B, équivalente à ¬A ∨ B (règle du sujet).
fn implies(left: bool, right: bool) -> bool {
    !left || right
}

/// Équivalence A ⇔ B : vraie quand les deux valeurs sont égales.
fn equivalent(left: bool, right: bool) -> bool {
    left == right
}

/// Version qui retourne l'erreur au lieu de l'afficher.
/// Les variables sont refusées : l'ex 03 n'autorise que `0` et `1`.
pub fn try_eval_formula(formula: &str) -> Result<bool, FormulaError> {
    let tree = parse_rpn(formula)?;
    evaluate(&tree, &Assignment::new())
}

/// Prototype imposé par le sujet.
///
/// Si la formule est invalide, le sujet laisse le comportement indéfini mais
/// encourage un message : on l'écrit sur stderr et on retourne `false`.
pub fn eval_formula(formula: &str) -> bool {
    try_eval_formula(formula).unwrap_or_else(|error| {
        eprintln!("Erreur : {error}");
        false
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_subject_examples() {
        assert!(!eval_formula("10&"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("11>"));
        assert!(!eval_formula("10="));
        assert!(eval_formula("1011||="));
    }

    #[test]
    fn evaluates_nested_subject_formulas() {
        assert!(eval_formula("10|1&")); // (⊤ ∨ ⊥) ∧ ⊤
        assert!(eval_formula("101|&")); // ⊤ ∧ (⊥ ∨ ⊤)
    }

    #[test]
    fn single_constants_and_negation() {
        assert!(eval_formula("1"));
        assert!(!eval_formula("0"));
        assert!(!eval_formula("1!"));
        assert!(eval_formula("0!"));
        assert!(eval_formula("1!!"));
    }

    #[test]
    fn truth_tables_of_each_operator() {
        // (gauche, droite) -> attendu, dans l'ordre 00, 01, 10, 11
        let cases: [(char, [bool; 4]); 5] = [
            ('&', [false, false, false, true]),
            ('|', [false, true, true, true]),
            ('^', [false, true, true, false]),
            ('>', [true, true, false, true]),
            ('=', [true, false, false, true]),
        ];
        for (operator, expected) in cases {
            for (index, (l, r)) in [('0', '0'), ('0', '1'), ('1', '0'), ('1', '1')]
                .into_iter()
                .enumerate()
            {
                let formula = format!("{l}{r}{operator}");
                assert_eq!(eval_formula(&formula), expected[index], "{formula}");
            }
        }
    }

    #[test]
    fn implication_is_not_symmetric() {
        assert!(!eval_formula("10>"));
        assert!(eval_formula("01>"));
    }

    #[test]
    fn variables_use_the_assignment() {
        let tree = parse_rpn("AB&!").unwrap();
        let mut assignment = Assignment::new();
        assignment.insert('A', true);
        assignment.insert('B', true);
        assert_eq!(evaluate(&tree, &assignment), Ok(false));
        assignment.insert('B', false);
        assert_eq!(evaluate(&tree, &assignment), Ok(true));
    }

    #[test]
    fn reports_undefined_variable() {
        assert_eq!(try_eval_formula("A1&"), Err(FormulaError::UndefinedVariable('A')));
    }

    #[test]
    fn reports_invalid_formulas_and_returns_false() {
        assert_eq!(try_eval_formula(""), Err(FormulaError::EmptyFormula));
        assert_eq!(try_eval_formula("1&"), Err(FormulaError::MissingOperand('&')));
        assert_eq!(try_eval_formula("11"), Err(FormulaError::TooManyOperands(2)));
        assert!(!eval_formula("1&"));
    }
}
