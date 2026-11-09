//! Exercice 09 : évaluation d'une formule dont les variables sont des ensembles.
//!
//! Correspondance logique / ensembles (sujet, section « Link between Logic and
//! Set Theory ») : `!` = complément, `|` = union, `&` = intersection.
//! L'ensemble « global » (par rapport auquel on prend le complément) est l'union
//! de tous les ensembles fournis. La lettre `A` désigne le 1er ensemble, `B` le 2e...

use crate::logic::{parse_rpn, Formula, FormulaError};
use std::collections::BTreeSet;

type IntSet = BTreeSet<i32>;

/// Convertit les listes d'entiers en vrais ensembles (sans doublon, triés).
fn to_int_sets(lists: &[Vec<i32>]) -> Vec<IntSet> {
    lists.iter().map(|list| list.iter().copied().collect()).collect()
}

/// Union de tous les ensembles : l'ensemble global.
fn universe_of(sets: &[IntSet]) -> IntSet {
    sets.iter().flatten().copied().collect()
}

/// Complément : les éléments de l'univers qui ne sont pas dans `set`.
fn complement(universe: &IntSet, set: &IntSet) -> IntSet {
    universe.difference(set).copied().collect()
}

fn union(first: &IntSet, second: &IntSet) -> IntSet {
    first.union(second).copied().collect()
}

fn intersection(first: &IntSet, second: &IntSet) -> IntSet {
    first.intersection(second).copied().collect()
}

/// Différence symétrique : dans exactement un des deux ensembles (équivalent de ⊕).
fn exclusive_union(first: &IntSet, second: &IntSet) -> IntSet {
    first.symmetric_difference(second).copied().collect()
}

/// `A ⇒ B` devient `¬A ∪ B` (condition matérielle).
fn implication(universe: &IntSet, first: &IntSet, second: &IntSet) -> IntSet {
    union(&complement(universe, first), second)
}

/// `A ⇔ B` : éléments dans les deux ensembles ou dans aucun, soit le complément de ⊕.
fn equivalence(universe: &IntSet, first: &IntSet, second: &IntSet) -> IntSet {
    complement(universe, &exclusive_union(first, second))
}

/// Retrouve l'ensemble associé à une lettre (`A` = index 0, `B` = index 1...).
fn set_for_variable(name: char, sets: &[IntSet]) -> Result<&IntSet, FormulaError> {
    let index = name as usize - 'A' as usize;
    sets.get(index).ok_or(FormulaError::UndefinedVariable(name))
}

/// Évalue l'arbre : même récursion que pour les booléens, mais sur des ensembles.
fn evaluate_on_sets(
    formula: &Formula,
    sets: &[IntSet],
    universe: &IntSet,
) -> Result<IntSet, FormulaError> {
    let recurse = |sub_formula: &Formula| evaluate_on_sets(sub_formula, sets, universe);

    match formula {
        Formula::Const(true) => Ok(universe.clone()),
        Formula::Const(false) => Ok(IntSet::new()),
        Formula::Var(name) => set_for_variable(*name, sets).cloned(),
        Formula::Not(operand) => Ok(complement(universe, &recurse(operand)?)),
        Formula::And(l, r) => Ok(intersection(&recurse(l)?, &recurse(r)?)),
        Formula::Or(l, r) => Ok(union(&recurse(l)?, &recurse(r)?)),
        Formula::Xor(l, r) => Ok(exclusive_union(&recurse(l)?, &recurse(r)?)),
        Formula::Implies(l, r) => Ok(implication(universe, &recurse(l)?, &recurse(r)?)),
        Formula::Equiv(l, r) => Ok(equivalence(universe, &recurse(l)?, &recurse(r)?)),
    }
}

/// Vérifie qu'il y a autant d'ensembles que de variables distinctes.
fn check_set_count(formula: &Formula, set_count: usize) -> Result<(), FormulaError> {
    let variable_count = formula.variables().len();
    if variable_count == set_count {
        Ok(())
    } else {
        Err(FormulaError::SetCountMismatch {
            variables: variable_count,
            sets: set_count,
        })
    }
}

/// Version qui retourne l'erreur au lieu de l'afficher. Résultat trié, sans doublon.
pub fn try_eval_set(formula: &str, sets: &[Vec<i32>]) -> Result<Vec<i32>, FormulaError> {
    let tree = parse_rpn(formula)?;
    check_set_count(&tree, sets.len())?;

    let sets = to_int_sets(sets);
    let universe = universe_of(&sets);
    let result = evaluate_on_sets(&tree, &sets, &universe)?;
    Ok(result.into_iter().collect())
}

/// Prototype imposé par le sujet. Cas invalide : message sur stderr, puis liste vide.
pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    try_eval_set(formula, &sets).unwrap_or_else(|error| {
        eprintln!("Erreur : {error}");
        Vec::new()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_subject_examples() {
        assert_eq!(eval_set("AB&", vec![vec![0, 1, 2], vec![0, 3, 4]]), vec![0]);
        assert_eq!(
            eval_set("AB|", vec![vec![0, 1, 2], vec![3, 4, 5]]),
            vec![0, 1, 2, 3, 4, 5]
        );
        assert_eq!(eval_set("A!", vec![vec![0, 1, 2]]), Vec::<i32>::new());
    }

    #[test]
    fn complement_is_relative_to_the_union_of_all_sets() {
        let sets = vec![vec![1, 2], vec![2, 3, 4]];
        // univers = {1, 2, 3, 4} ; ¬A = {3, 4} ; ¬A ∩ B = {3, 4}
        assert_eq!(eval_set("A!B&", sets.clone()), vec![3, 4]);
        // ¬B = {1} ; ¬B ∩ A = {1}
        assert_eq!(eval_set("B!A&", sets), vec![1]);
    }

    #[test]
    fn derived_operators() {
        let sets = vec![vec![1, 2, 3], vec![3, 4]];
        assert_eq!(eval_set("AB^", sets.clone()), vec![1, 2, 4]);
        assert_eq!(eval_set("AB>", sets.clone()), vec![3, 4]); // ¬A ∪ B
        assert_eq!(eval_set("AB=", sets), vec![3]); // dans les deux ou aucun
    }

    #[test]
    fn de_morgan_holds_on_sets() {
        let sets = vec![vec![1, 2, 3], vec![3, 4], vec![0, 3]];
        assert_eq!(eval_set("AB&!", sets.clone()), eval_set("A!B!|", sets.clone()));
        assert_eq!(eval_set("AB|!", sets.clone()), eval_set("A!B!&", sets));
    }

    #[test]
    fn duplicates_in_input_and_negative_numbers_are_handled() {
        assert_eq!(eval_set("A", vec![vec![3, -1, 3]]), vec![-1, 3]);
    }

    #[test]
    fn empty_sets() {
        assert_eq!(eval_set("A!", vec![vec![]]), Vec::<i32>::new());
        assert_eq!(eval_set("AB|", vec![vec![], vec![5]]), vec![5]);
    }

    #[test]
    fn constants_use_the_universe() {
        assert_eq!(eval_set("A1&", vec![vec![1, 2]]), vec![1, 2]);
        assert_eq!(eval_set("A0|", vec![vec![1, 2]]), vec![1, 2]);
    }

    #[test]
    fn reports_wrong_number_of_sets() {
        assert_eq!(
            try_eval_set("AB&", &[vec![1]]),
            Err(FormulaError::SetCountMismatch { variables: 2, sets: 1 })
        );
        assert!(eval_set("AB&", vec![vec![1]]).is_empty());
    }

    #[test]
    fn reports_variable_without_matching_set() {
        // 2 variables (A et C) pour 2 ensembles : C n'a pas d'ensemble (3e lettre).
        assert_eq!(
            try_eval_set("AC&", &[vec![1], vec![2]]),
            Err(FormulaError::UndefinedVariable('C'))
        );
    }

    #[test]
    fn reports_invalid_formula() {
        assert_eq!(try_eval_set("A&", &[vec![1]]), Err(FormulaError::MissingOperand('&')));
    }
}
