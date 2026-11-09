//! Exercice 06 : forme normale conjonctive (CNF).
//!
//! Une CNF est un `&` de clauses, chaque clause étant un `|` de littéraux
//! (variable ou variable niée). Chaîne de calcul :
//!   `Formula` --to_nnf--> NNF --convert_nnf_to_cnf (distributivité)--> CNF
//!   --normalize_layout--> CNF écrite de façon canonique --to_rpn--> `String`.
//!
//! La taille de sortie peut croître exponentiellement : le sujet l'autorise.

use super::error::FormulaError;
use super::formula::Formula;
use super::nnf::to_nnf;
use super::parser::parse_rpn;
use super::printer::to_rpn;

/// Constructeur de nœud binaire (`Formula::And` ou `Formula::Or`).
type BinaryBuilder = fn(Box<Formula>, Box<Formula>) -> Formula;

/// Convertit l'arbre en CNF.
pub fn to_cnf(formula: &Formula) -> Formula {
    let nnf = to_nnf(formula);
    let cnf = convert_nnf_to_cnf(&nnf);
    normalize_layout(&cnf)
}

/// Transforme une NNF en CNF : les `&` remontent au-dessus des `|`.
///
/// On convertit d'abord les enfants, puis, pour un `|`, on distribue.
fn convert_nnf_to_cnf(nnf: &Formula) -> Formula {
    match nnf {
        Formula::And(left, right) => Formula::And(
            Box::new(convert_nnf_to_cnf(left)),
            Box::new(convert_nnf_to_cnf(right)),
        ),
        Formula::Or(left, right) => {
            let left_cnf = convert_nnf_to_cnf(left);
            let right_cnf = convert_nnf_to_cnf(right);
            distribute_or_over_and(left_cnf, right_cnf)
        }
        literal_or_constant => literal_or_constant.clone(),
    }
}

/// `left ∨ right` où `left` et `right` sont déjà en CNF.
///
/// Distributivité du sujet : `(A ∧ B) ∨ C ⇔ (A ∨ C) ∧ (B ∨ C)` (et symétriquement).
/// Si aucun côté n'est un `&`, c'est déjà une clause : on garde le `|`.
fn distribute_or_over_and(left: Formula, right: Formula) -> Formula {
    match (left, right) {
        (Formula::And(a, b), right) => Formula::And(
            Box::new(distribute_or_over_and(*a, right.clone())),
            Box::new(distribute_or_over_and(*b, right)),
        ),
        (left, Formula::And(c, d)) => Formula::And(
            Box::new(distribute_or_over_and(left.clone(), *c)),
            Box::new(distribute_or_over_and(left, *d)),
        ),
        (left, right) => Formula::Or(Box::new(left), Box::new(right)),
    }
}

/// Réécrit la CNF avec des chaînes de `&` et de `|` imbriquées à DROITE.
///
/// Sémantiquement inutile (`&` et `|` sont associatifs) mais cela donne exactement
/// les écritures du sujet : `ABCD|||` et non `AB|C|D|`.
fn normalize_layout(cnf: &Formula) -> Formula {
    let mut clauses = Vec::new();
    flatten_chain(cnf, split_and, &mut clauses);

    let normalized_clauses: Vec<Formula> = clauses.into_iter().map(normalize_clause).collect();
    rebuild_right_nested(normalized_clauses, Formula::And)
}

/// Normalise une clause (une chaîne de `|`).
fn normalize_clause(clause: &Formula) -> Formula {
    let mut literals = Vec::new();
    flatten_chain(clause, split_or, &mut literals);

    let owned_literals: Vec<Formula> = literals.into_iter().cloned().collect();
    rebuild_right_nested(owned_literals, Formula::Or)
}

/// Si `formula` est un `&`, retourne ses deux enfants.
fn split_and(formula: &Formula) -> Option<(&Formula, &Formula)> {
    match formula {
        Formula::And(left, right) => Some((left, right)),
        _ => None,
    }
}

/// Si `formula` est un `|`, retourne ses deux enfants.
fn split_or(formula: &Formula) -> Option<(&Formula, &Formula)> {
    match formula {
        Formula::Or(left, right) => Some((left, right)),
        _ => None,
    }
}

/// Aplatit une chaîne d'un même opérateur en liste, de gauche à droite.
/// `split` dit si un nœud appartient à la chaîne (et donne ses enfants).
fn flatten_chain<'a>(
    formula: &'a Formula,
    split: fn(&Formula) -> Option<(&Formula, &Formula)>,
    out: &mut Vec<&'a Formula>,
) {
    match split(formula) {
        Some((left, right)) => {
            flatten_chain(left, split, out);
            flatten_chain(right, split, out);
        }
        None => out.push(formula),
    }
}

/// `[a, b, c]` devient `join(a, join(b, c))`. La liste ne doit pas être vide
/// (`flatten_chain` y met toujours au moins un élément).
fn rebuild_right_nested(operands: Vec<Formula>, join: BinaryBuilder) -> Formula {
    operands
        .into_iter()
        .rev()
        .reduce(|accumulated, previous| join(Box::new(previous), Box::new(accumulated)))
        .expect("une chaîne contient au moins un opérande")
}

/// Version qui retourne l'erreur au lieu de l'afficher.
pub fn try_conjunctive_normal_form(formula: &str) -> Result<String, FormulaError> {
    let tree = parse_rpn(formula)?;
    Ok(to_rpn(&to_cnf(&tree)))
}

/// Prototype imposé par le sujet. Formule invalide : message sur stderr, puis chaîne vide.
pub fn conjunctive_normal_form(formula: &str) -> String {
    try_conjunctive_normal_form(formula).unwrap_or_else(|error| {
        eprintln!("Erreur : {error}");
        String::new()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::test_support::{are_equivalent, is_cnf};

    #[test]
    fn matches_subject_examples() {
        assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
        assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
        assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
        assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
        assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
        assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
        assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
    }

    #[test]
    fn matches_distribution_example_of_the_subject() {
        assert_eq!(conjunctive_normal_form("ABCD&|&"), "ABC|BD|&&");
    }

    #[test]
    fn distributes_on_both_sides() {
        // (A ∧ B) ∨ (C ∧ D) = (A∨C) ∧ (A∨D) ∧ (B∨C) ∧ (B∨D)
        let result = conjunctive_normal_form("AB&CD&|");
        let tree = parse_rpn(&result).unwrap();
        assert!(is_cnf(&tree));
        assert!(are_equivalent(&parse_rpn("AB&CD&|").unwrap(), &tree));
    }

    #[test]
    fn single_variable_and_constants() {
        assert_eq!(conjunctive_normal_form("A"), "A");
        assert_eq!(conjunctive_normal_form("A!"), "A!");
        assert_eq!(conjunctive_normal_form("1"), "1");
    }

    #[test]
    fn result_is_cnf_and_equivalent_for_every_operator_and_nesting() {
        let formulas = [
            "AB^", "AB>", "AB=", "AB=!", "AB^C>", "AB|CD&^", "AB>C=D^", "ABC&|!",
            "AB&C|D&E|", "A!B!&C^",
        ];
        for rpn in formulas {
            let original = parse_rpn(rpn).unwrap();
            let converted = to_cnf(&original);
            assert!(is_cnf(&converted), "{rpn} -> {}", to_rpn(&converted));
            assert!(are_equivalent(&original, &converted), "{rpn}");
        }
    }

    #[test]
    fn invalid_formula_gives_empty_string() {
        assert_eq!(try_conjunctive_normal_form("AB"), Err(FormulaError::TooManyOperands(2)));
        assert_eq!(conjunctive_normal_form(""), "");
    }
}
