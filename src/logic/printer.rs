//! Écriture d'un `Formula` en notation polonaise inverse (l'inverse de `parser`).
//!
//! Parcours « postfixe » : on écrit d'abord les opérandes (gauche puis droite),
//! puis le symbole de l'opérateur.

use super::formula::*;

/// Retourne la formule RPN correspondant à l'arbre.
pub fn to_rpn(formula: &Formula) -> String {
    let mut output = String::new();
    write_rpn(formula, &mut output);
    output
}

/// Ajoute à `output` la RPN de `formula`.
fn write_rpn(formula: &Formula, output: &mut String) {
    match formula {
        Formula::Const(false) => output.push(SYMBOL_FALSE),
        Formula::Const(true) => output.push(SYMBOL_TRUE),
        Formula::Var(name) => output.push(*name),
        Formula::Not(operand) => write_unary(SYMBOL_NOT, operand, output),
        Formula::And(l, r) => write_binary(SYMBOL_AND, l, r, output),
        Formula::Or(l, r) => write_binary(SYMBOL_OR, l, r, output),
        Formula::Xor(l, r) => write_binary(SYMBOL_XOR, l, r, output),
        Formula::Implies(l, r) => write_binary(SYMBOL_IMPLIES, l, r, output),
        Formula::Equiv(l, r) => write_binary(SYMBOL_EQUIV, l, r, output),
    }
}

fn write_unary(symbol: char, operand: &Formula, output: &mut String) {
    write_rpn(operand, output);
    output.push(symbol);
}

fn write_binary(symbol: char, left: &Formula, right: &Formula, output: &mut String) {
    write_rpn(left, output);
    write_rpn(right, output);
    output.push(symbol);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::parser::parse_rpn;

    #[test]
    fn round_trip_keeps_the_formula_unchanged() {
        for rpn in ["1", "A", "A!", "10&", "101|&", "AB>", "AB=C^", "AB&!C!|", "11>!0="] {
            let tree = parse_rpn(rpn).unwrap();
            assert_eq!(to_rpn(&tree), rpn);
        }
    }

    #[test]
    fn writes_operands_before_operator() {
        let tree = Formula::Implies(
            Box::new(Formula::Var('A')),
            Box::new(Formula::Const(false)),
        );
        assert_eq!(to_rpn(&tree), "A0>");
    }
}
