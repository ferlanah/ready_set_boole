//! Lecture d'une formule en notation polonaise inverse (RPN) vers un `Formula`.
//!
//! Algorithme classique à pile, en O(n) :
//! - une constante ou une variable est empilée ;
//! - `!` dépile 1 opérande et empile `Not(opérande)` ;
//! - un opérateur binaire dépile 2 opérandes (le DERNIER dépilé est l'opérande
//!   de gauche) et empile le nœud correspondant ;
//! - à la fin, la pile doit contenir exactement 1 arbre : la racine.

use super::error::FormulaError;
use super::formula::*;

/// Constructeur d'un nœud binaire (ex. `Formula::And`).
type BinaryBuilder = fn(Box<Formula>, Box<Formula>) -> Formula;

/// Transforme une formule RPN en arbre, ou explique pourquoi elle est invalide.
pub fn parse_rpn(rpn: &str) -> Result<Formula, FormulaError> {
    if rpn.is_empty() {
        return Err(FormulaError::EmptyFormula);
    }

    let mut stack: Vec<Formula> = Vec::new();
    for symbol in rpn.chars() {
        apply_symbol(&mut stack, symbol)?;
    }
    extract_single_root(stack)
}

/// Applique UN symbole à la pile (empile une feuille ou combine des opérandes).
fn apply_symbol(stack: &mut Vec<Formula>, symbol: char) -> Result<(), FormulaError> {
    if let Some(leaf) = symbol_to_leaf(symbol) {
        stack.push(leaf);
    } else if symbol == SYMBOL_NOT {
        let operand = pop_operand(stack, symbol)?;
        stack.push(Formula::Not(Box::new(operand)));
    } else if let Some(build) = symbol_to_binary_builder(symbol) {
        // Le sommet de pile est l'opérande de DROITE, puisqu'il a été écrit en dernier.
        let right = pop_operand(stack, symbol)?;
        let left = pop_operand(stack, symbol)?;
        stack.push(build(Box::new(left), Box::new(right)));
    } else {
        return Err(FormulaError::UnknownSymbol(symbol));
    }
    Ok(())
}

/// Reconnaît les symboles sans opérande : `0`, `1` et les variables `A`..`Z`.
fn symbol_to_leaf(symbol: char) -> Option<Formula> {
    match symbol {
        SYMBOL_FALSE => Some(Formula::Const(false)),
        SYMBOL_TRUE => Some(Formula::Const(true)),
        'A'..='Z' => Some(Formula::Var(symbol)),
        _ => None,
    }
}

/// Reconnaît les opérateurs binaires et donne le constructeur de nœud associé.
fn symbol_to_binary_builder(symbol: char) -> Option<BinaryBuilder> {
    match symbol {
        SYMBOL_AND => Some(Formula::And),
        SYMBOL_OR => Some(Formula::Or),
        SYMBOL_XOR => Some(Formula::Xor),
        SYMBOL_IMPLIES => Some(Formula::Implies),
        SYMBOL_EQUIV => Some(Formula::Equiv),
        _ => None,
    }
}

/// Dépile un opérande pour `operator`, ou signale qu'il en manque.
fn pop_operand(stack: &mut Vec<Formula>, operator: char) -> Result<Formula, FormulaError> {
    stack.pop().ok_or(FormulaError::MissingOperand(operator))
}

/// Vérifie qu'il reste exactement un arbre et le retourne.
fn extract_single_root(mut stack: Vec<Formula>) -> Result<Formula, FormulaError> {
    match stack.len() {
        1 => Ok(stack.remove(0)),
        count => Err(FormulaError::TooManyOperands(count)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boxed(f: Formula) -> Box<Formula> {
        Box::new(f)
    }

    #[test]
    fn parses_constants_and_variables() {
        assert_eq!(parse_rpn("1"), Ok(Formula::Const(true)));
        assert_eq!(parse_rpn("0"), Ok(Formula::Const(false)));
        assert_eq!(parse_rpn("Z"), Ok(Formula::Var('Z')));
    }

    #[test]
    fn parses_negation() {
        assert_eq!(parse_rpn("1!"), Ok(Formula::Not(boxed(Formula::Const(true)))));
    }

    #[test]
    fn left_operand_is_the_one_written_first() {
        // "10>" = 1 => 0, et non 0 => 1.
        assert_eq!(
            parse_rpn("10>"),
            Ok(Formula::Implies(
                boxed(Formula::Const(true)),
                boxed(Formula::Const(false))
            ))
        );
    }

    #[test]
    fn parses_nested_formula_from_subject() {
        // 101|& = ⊤ ∧ (⊥ ∨ ⊤)
        let expected = Formula::And(
            boxed(Formula::Const(true)),
            boxed(Formula::Or(
                boxed(Formula::Const(false)),
                boxed(Formula::Const(true)),
            )),
        );
        assert_eq!(parse_rpn("101|&"), Ok(expected));
    }

    #[test]
    fn parses_every_binary_operator() {
        for symbol in ['&', '|', '^', '>', '='] {
            assert!(parse_rpn(&format!("AB{symbol}")).is_ok(), "{symbol}");
        }
    }

    #[test]
    fn rejects_empty_formula() {
        assert_eq!(parse_rpn(""), Err(FormulaError::EmptyFormula));
    }

    #[test]
    fn rejects_unknown_symbols() {
        assert_eq!(parse_rpn("1x&"), Err(FormulaError::UnknownSymbol('x')));
        assert_eq!(parse_rpn("1 0&"), Err(FormulaError::UnknownSymbol(' ')));
        assert_eq!(parse_rpn("a"), Err(FormulaError::UnknownSymbol('a')));
    }

    #[test]
    fn rejects_missing_operands() {
        assert_eq!(parse_rpn("!"), Err(FormulaError::MissingOperand('!')));
        assert_eq!(parse_rpn("1&"), Err(FormulaError::MissingOperand('&')));
        assert_eq!(parse_rpn("&"), Err(FormulaError::MissingOperand('&')));
    }

    #[test]
    fn rejects_leftover_operands() {
        assert_eq!(parse_rpn("11"), Err(FormulaError::TooManyOperands(2)));
        assert_eq!(parse_rpn("10&1"), Err(FormulaError::TooManyOperands(2)));
    }
}
