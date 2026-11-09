//! Démonstration des exercices (affichage uniquement, aucun calcul ici).

use ready_set_boole::*;

fn main() {
    println!("== Ex 00 : adder ==");
    println!("adder(5, 7) = {}", adder(5, 7));

    println!("== Ex 01 : multiplier ==");
    println!("multiplier(6, 7) = {}", multiplier(6, 7));

    println!("== Ex 02 : gray_code ==");
    for n in 0..=8 {
        println!("gray_code({n}) = {}", gray_code(n));
    }

    println!("== Ex 03 : eval_formula ==");
    for formula in ["10&", "10|", "11>", "10=", "1011||="] {
        println!("eval_formula({formula:?}) = {}", eval_formula(formula));
    }

    println!("== Ex 04 : print_truth_table(\"AB&C|\") ==");
    print_truth_table("AB&C|");

    println!("== Ex 05 : negation_normal_form ==");
    for formula in ["AB&!", "AB|!", "AB>", "AB=", "AB|C&!"] {
        println!("{formula} -> {}", negation_normal_form(formula));
    }

    println!("== Ex 06 : conjunctive_normal_form ==");
    for formula in ["AB&!", "AB|C&", "AB|C|D|", "ABCD&|&"] {
        println!("{formula} -> {}", conjunctive_normal_form(formula));
    }

    println!("== Ex 07 : sat ==");
    for formula in ["AB|", "AB&", "AA!&", "AA^"] {
        println!("sat({formula:?}) = {}", sat(formula));
    }

    println!("== Ex 08 : powerset ==");
    println!("powerset([1, 2, 3]) = {:?}", powerset(vec![1, 2, 3]));

    println!("== Ex 09 : eval_set ==");
    println!("AB& = {:?}", eval_set("AB&", vec![vec![0, 1, 2], vec![0, 3, 4]]));
    println!("AB| = {:?}", eval_set("AB|", vec![vec![0, 1, 2], vec![3, 4, 5]]));
    println!("A!  = {:?}", eval_set("A!", vec![vec![0, 1, 2]]));

    println!("== Ex 10-11 : map / reverse_map ==");
    let value = map(3, 5);
    println!("map(3, 5) = {value}");
    println!("reverse_map({value}) = {:?}", reverse_map(value));
}
