//! Outils réservés aux tests : vérifier qu'une formule est en NNF / CNF
//! et que deux formules sont équivalentes (mêmes valeurs sur toutes les lignes).

use super::evaluator::evaluate;
use super::formula::Formula;
use super::truth_table::{assignment_for_row, row_count};

/// Deux formules sont équivalentes si elles ont la même valeur sur toutes les affectations
/// de l'union de leurs variables.
pub fn are_equivalent(first: &Formula, second: &Formula) -> bool {
    let mut variables = first.variables();
    variables.extend(second.variables());
    variables.sort();
    variables.dedup();

    (0..row_count(variables.len())).all(|row| {
        let assignment = assignment_for_row(&variables, row);
        evaluate(first, &assignment) == evaluate(second, &assignment)
    })
}

/// NNF : seulement `Not` (devant une variable), `And`, `Or`.
pub fn is_nnf(formula: &Formula) -> bool {
    match formula {
        Formula::Const(_) | Formula::Var(_) => true,
        Formula::Not(operand) => matches!(**operand, Formula::Var(_)),
        Formula::And(l, r) | Formula::Or(l, r) => is_nnf(l) && is_nnf(r),
        Formula::Xor(..) | Formula::Implies(..) | Formula::Equiv(..) => false,
    }
}

/// CNF : NNF dans laquelle aucun `Or` ne contient un `And` en dessous.
pub fn is_cnf(formula: &Formula) -> bool {
    is_nnf(formula) && and_never_below_or(formula)
}

fn and_never_below_or(formula: &Formula) -> bool {
    match formula {
        Formula::And(l, r) => and_never_below_or(l) && and_never_below_or(r),
        Formula::Or(l, r) => !contains_and(l) && !contains_and(r),
        _ => true,
    }
}

fn contains_and(formula: &Formula) -> bool {
    match formula {
        Formula::And(..) => true,
        Formula::Or(l, r) => contains_and(l) || contains_and(r),
        Formula::Not(operand) => contains_and(operand),
        _ => false,
    }
}
