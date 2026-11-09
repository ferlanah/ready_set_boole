//! Exercice 04 : table de vérité.
//!
//! Chaîne de calcul : RPN --parse--> `Formula` --variables()--> colonnes
//! puis, pour chaque ligne : numéro de ligne --assignment_for_row--> affectation
//! --evaluate--> résultat. Le calcul (ce fichier, partie haute) est séparé du
//! rendu texte (fonctions `render_*`) et de l'écriture sur la sortie.
//! Complexité : 2^n lignes, chacune évaluée en O(taille de la formule).

use super::error::FormulaError;
use super::evaluator::{evaluate, Assignment};
use super::formula::Formula;
use super::parser::parse_rpn;

/// Nombre de lignes de la table : 2^(nombre de variables).
pub(super) fn row_count(variable_count: usize) -> u32 {
    1u32 << variable_count
}

/// Construit l'affectation de la ligne numéro `row_index`.
///
/// Le numéro de ligne, écrit en binaire sur `n` bits, donne les valeurs :
/// le bit de poids FORT correspond à la PREMIÈRE variable. Ainsi, pour 3 variables,
/// les lignes 0..7 donnent 000, 001, 010, ..., 111, l'ordre exigé par le sujet.
pub(super) fn assignment_for_row(variables: &[char], row_index: u32) -> Assignment {
    let variable_count = variables.len();
    let mut assignment = Assignment::new();

    for (position, &name) in variables.iter().enumerate() {
        let bit_position = variable_count - 1 - position;
        let bit = (row_index >> bit_position) & 1;
        assignment.insert(name, bit == 1);
    }
    assignment
}

/// Valeur de la formule sur la ligne `row_index`.
pub(super) fn evaluate_row(
    formula: &Formula,
    variables: &[char],
    row_index: u32,
) -> Result<bool, FormulaError> {
    let assignment = assignment_for_row(variables, row_index);
    evaluate(formula, &assignment)
}

fn bit_char(value: bool) -> char {
    if value {
        '1'
    } else {
        '0'
    }
}

/// Met en forme une ligne de cellules : `| a | b | c |`.
fn render_cells(cells: &[char]) -> String {
    let texts: Vec<String> = cells.iter().map(|cell| cell.to_string()).collect();
    format!("| {} |", texts.join(" | "))
}

/// En-tête : les noms des variables puis `=` pour le résultat.
fn render_header(variables: &[char]) -> String {
    let mut cells = variables.to_vec();
    cells.push('=');
    render_cells(&cells)
}

/// Ligne de séparation : `|---|---|...` avec une colonne par variable + le résultat.
fn render_separator(variable_count: usize) -> String {
    let column_count = variable_count + 1;
    format!("|{}", "---|".repeat(column_count))
}

/// Ligne de données : les valeurs des variables puis le résultat.
fn render_row(variable_values: &[bool], result: bool) -> String {
    let mut cells: Vec<char> = variable_values.iter().map(|&v| bit_char(v)).collect();
    cells.push(bit_char(result));
    render_cells(&cells)
}

/// Ligne de données complète pour le numéro de ligne `row_index`.
fn render_data_row(
    tree: &Formula,
    variables: &[char],
    row_index: u32,
) -> Result<String, FormulaError> {
    let assignment = assignment_for_row(variables, row_index);
    let variable_values: Vec<bool> = variables.iter().map(|name| assignment[name]).collect();
    let result = evaluate(tree, &assignment)?;
    Ok(render_row(&variable_values, result))
}

/// Texte complet de la table (chaque ligne se termine par `\n`).
pub fn render_truth_table(formula: &str) -> Result<String, FormulaError> {
    let tree = parse_rpn(formula)?;
    let variables = tree.variables();

    let mut lines = vec![render_header(&variables), render_separator(variables.len())];
    for row_index in 0..row_count(variables.len()) {
        lines.push(render_data_row(&tree, &variables, row_index)?);
    }

    let mut text = lines.join("\n");
    text.push('\n');
    Ok(text)
}

/// Prototype imposé par le sujet : écrit la table sur la sortie standard.
/// En cas de formule invalide, écrit l'erreur sur stderr et n'affiche rien.
pub fn print_truth_table(formula: &str) {
    match render_truth_table(formula) {
        Ok(table) => print!("{table}"),
        Err(error) => eprintln!("Erreur : {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_subject_output_exactly() {
        let expected = "\
| A | B | C | = |
|---|---|---|---|
| 0 | 0 | 0 | 0 |
| 0 | 0 | 1 | 1 |
| 0 | 1 | 0 | 0 |
| 0 | 1 | 1 | 1 |
| 1 | 0 | 0 | 0 |
| 1 | 0 | 1 | 1 |
| 1 | 1 | 0 | 1 |
| 1 | 1 | 1 | 1 |
";
        assert_eq!(render_truth_table("AB&C|").unwrap(), expected);
    }

    #[test]
    fn row_numbering_gives_binary_order() {
        let variables = ['A', 'B', 'C'];
        let assignment = assignment_for_row(&variables, 0b101);
        assert!(assignment[&'A'] && !assignment[&'B'] && assignment[&'C']);
        assert_eq!(row_count(3), 8);
        assert_eq!(row_count(0), 1);
    }

    #[test]
    fn single_variable_and_repeated_variable() {
        assert_eq!(
            render_truth_table("A!").unwrap(),
            "| A | = |\n|---|---|\n| 0 | 1 |\n| 1 | 0 |\n"
        );
        // A apparaît 2 fois mais n'a qu'une colonne.
        assert_eq!(
            render_truth_table("AA^").unwrap(),
            "| A | = |\n|---|---|\n| 0 | 0 |\n| 1 | 0 |\n"
        );
    }

    #[test]
    fn formula_without_variables_has_a_single_row() {
        assert_eq!(render_truth_table("10|").unwrap(), "| = |\n|---|\n| 1 |\n");
    }

    #[test]
    fn variables_are_sorted_alphabetically_whatever_the_formula_order() {
        let table = render_truth_table("ZA>").unwrap();
        assert!(table.starts_with("| A | Z | = |\n"));
    }

    #[test]
    fn handles_26_variables_header() {
        let header = render_header(&('A'..='Z').collect::<Vec<_>>());
        assert!(header.starts_with("| A | B |") && header.ends_with("| Z | = |"));
        assert_eq!(row_count(26), 67_108_864);
    }

    #[test]
    fn invalid_formula_returns_an_error() {
        assert_eq!(render_truth_table("A&"), Err(FormulaError::MissingOperand('&')));
        assert_eq!(render_truth_table(""), Err(FormulaError::EmptyFormula));
    }
}
