//! L'arbre syntaxique (AST) d'une formule propositionnelle.
//!
//! Exemple : `101|&` (⊤ ∧ (⊥ ∨ ⊤)) devient
//! ```text
//!   And
//!   ├─ Const(true)
//!   └─ Or
//!      ├─ Const(false)
//!      └─ Const(true)
//! ```

use std::collections::BTreeSet;

// Alphabet du sujet : un seul endroit où chaque symbole est défini,
// partagé par le parseur (lecture) et l'afficheur (écriture).
pub const SYMBOL_FALSE: char = '0';
pub const SYMBOL_TRUE: char = '1';
pub const SYMBOL_NOT: char = '!';
pub const SYMBOL_AND: char = '&';
pub const SYMBOL_OR: char = '|';
pub const SYMBOL_XOR: char = '^';
pub const SYMBOL_IMPLIES: char = '>';
pub const SYMBOL_EQUIV: char = '=';

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Formula {
    /// `0` (⊥) ou `1` (⊤).
    Const(bool),
    /// Une variable `A`..`Z`.
    Var(char),
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Xor(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    Equiv(Box<Formula>, Box<Formula>),
}

impl Formula {
    /// Variables distinctes de la formule, triées par ordre alphabétique.
    ///
    /// Utilisée par la table de vérité (colonnes) et par SAT (nombre de lignes à essayer).
    pub fn variables(&self) -> Vec<char> {
        let mut found = BTreeSet::new();
        self.collect_variables_into(&mut found);
        found.into_iter().collect()
    }

    /// Parcourt l'arbre et ajoute chaque variable rencontrée à `found`.
    fn collect_variables_into(&self, found: &mut BTreeSet<char>) {
        match self {
            Formula::Const(_) => {}
            Formula::Var(name) => {
                found.insert(*name);
            }
            Formula::Not(operand) => operand.collect_variables_into(found),
            Formula::And(left, right)
            | Formula::Or(left, right)
            | Formula::Xor(left, right)
            | Formula::Implies(left, right)
            | Formula::Equiv(left, right) => {
                left.collect_variables_into(found);
                right.collect_variables_into(found);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn var(c: char) -> Box<Formula> {
        Box::new(Formula::Var(c))
    }

    #[test]
    fn constants_have_no_variables() {
        assert!(Formula::Const(true).variables().is_empty());
    }

    #[test]
    fn variables_are_distinct_and_sorted() {
        // (B & A) | !B
        let formula = Formula::Or(
            Box::new(Formula::And(var('B'), var('A'))),
            Box::new(Formula::Not(var('B'))),
        );
        assert_eq!(formula.variables(), vec!['A', 'B']);
    }
}
