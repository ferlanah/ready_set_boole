//! Erreurs possibles lors de la lecture ou de l'évaluation d'une formule.

use std::fmt;

/// Pourquoi une formule n'a pas pu être lue ou évaluée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormulaError {
    /// La chaîne est vide.
    EmptyFormula,
    /// Un caractère n'appartient pas à l'alphabet du sujet.
    UnknownSymbol(char),
    /// Un opérateur n'a pas assez d'opérandes dans la pile (ex. `1&`).
    MissingOperand(char),
    /// Plusieurs formules indépendantes restent à la fin (ex. `11`).
    TooManyOperands(usize),
    /// Une variable n'a pas de valeur dans l'affectation fournie.
    UndefinedVariable(char),
    /// Le nombre d'ensembles fournis diffère du nombre de variables (ex 09).
    SetCountMismatch { variables: usize, sets: usize },
}

impl fmt::Display for FormulaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormulaError::EmptyFormula => write!(f, "formule vide"),
            FormulaError::UnknownSymbol(c) => write!(f, "symbole inconnu : '{c}'"),
            FormulaError::MissingOperand(c) => {
                write!(f, "opérande manquant pour l'opérateur '{c}'")
            }
            FormulaError::TooManyOperands(n) => {
                write!(f, "formule invalide : {n} expressions restantes au lieu d'une")
            }
            FormulaError::UndefinedVariable(c) => {
                write!(f, "la variable '{c}' n'a pas de valeur")
            }
            FormulaError::SetCountMismatch { variables, sets } => write!(
                f,
                "la formule a {variables} variable(s) mais {sets} ensemble(s) sont fournis"
            ),
        }
    }
}

impl std::error::Error for FormulaError {}
