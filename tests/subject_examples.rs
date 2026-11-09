//! Les exemples du sujet, utilisés tels quels via l'API publique de la bibliothèque.

use ready_set_boole::*;

#[test]
fn ex00_to_ex02_arithmetic() {
    assert_eq!(adder(5, 7), 12);
    assert_eq!(multiplier(6, 7), 42);
    let expected = [0, 1, 3, 2, 6, 7, 5, 4, 12];
    for (n, gray) in expected.iter().enumerate() {
        assert_eq!(gray_code(n as u32), *gray);
    }
}

#[test]
fn ex03_eval_formula() {
    assert!(!eval_formula("10&"));
    assert!(eval_formula("10|"));
    assert!(eval_formula("11>"));
    assert!(!eval_formula("10="));
    assert!(eval_formula("1011||="));
}

#[test]
fn ex05_negation_normal_form() {
    assert_eq!(negation_normal_form("AB&!"), "A!B!|");
    assert_eq!(negation_normal_form("AB|!"), "A!B!&");
    assert_eq!(negation_normal_form("AB>"), "A!B|");
    assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
    assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
}

#[test]
fn ex06_conjunctive_normal_form() {
    assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
    assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
    assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
    assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
    assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
    assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
    assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
    assert_eq!(conjunctive_normal_form("ABCD&|&"), "ABC|BD|&&");
}

#[test]
fn ex07_sat() {
    assert!(sat("AB|"));
    assert!(sat("AB&"));
    assert!(!sat("AA!&"));
    assert!(!sat("AA^"));
}

#[test]
fn ex08_powerset_has_two_to_the_n_parts() {
    assert_eq!(powerset(vec![1, 2, 3]).len(), 8);
}

#[test]
fn ex09_eval_set() {
    assert_eq!(eval_set("AB&", vec![vec![0, 1, 2], vec![0, 3, 4]]), vec![0]);
    assert_eq!(
        eval_set("AB|", vec![vec![0, 1, 2], vec![3, 4, 5]]),
        vec![0, 1, 2, 3, 4, 5]
    );
    assert_eq!(eval_set("A!", vec![vec![0, 1, 2]]), Vec::<i32>::new());
}

#[test]
fn ex10_ex11_map_and_reverse_map_are_inverse() {
    for (x, y) in [(0, 0), (1, 0), (0, 1), (3, 5), (65_535, 65_535), (12_345, 54_321)] {
        assert_eq!(reverse_map(map(x, y)), (x, y));
    }
}
