# Ready, Set, Boole!

[English](#english) · [Français](#français)

---

## English

A Rust implementation of exercises 00 to 11 of the subject "Ready, Set, Boole! — An introduction to Boolean Algebra" (version 2.1).

No external dependencies: only the standard library is used, and no mathematical function (`pow`, `sqrt`, ...) is called.

### Usage

```sh
cargo run      # demo of every exercise
cargo test     # unit tests (in each module) + the subject's examples (tests/)
cargo clippy --all-targets
```

### Subject functions

All of them are available at the crate root (`use ready_set_boole::*;`) with the prototypes required by the subject.

| Ex | Prototype | Module |
|---|---|---|
| 00 | `fn adder(a: u32, b: u32) -> u32` | `arithmetic/adder.rs` |
| 01 | `fn multiplier(a: u32, b: u32) -> u32` | `arithmetic/multiplier.rs` |
| 02 | `fn gray_code(n: u32) -> u32` | `arithmetic/gray_code.rs` |
| 03 | `fn eval_formula(formula: &str) -> bool` | `logic/evaluator.rs` |
| 04 | `fn print_truth_table(formula: &str)` | `logic/truth_table.rs` |
| 05 | `fn negation_normal_form(formula: &str) -> String` | `logic/nnf.rs` |
| 06 | `fn conjunctive_normal_form(formula: &str) -> String` | `logic/cnf.rs` |
| 07 | `fn sat(formula: &str) -> bool` | `logic/sat.rs` |
| 08 | `fn powerset(set: Vec<i32>) -> Vec<Vec<i32>>` | `sets/powerset.rs` |
| 09 | `fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32>` | `sets/set_evaluator.rs` |
| 10 | `fn map(x: u16, y: u16) -> f64` | `curve/mapping.rs` |
| 11 | `fn reverse_map(n: f64) -> (u16, u16)` | `curve/mapping.rs` |

### Architecture

```
src/
├── lib.rs, main.rs        main.rs: demo (printing only)
├── arithmetic/            ex 00-02: bitwise operations
├── logic/                 ex 03-07: propositional logic
│   ├── error.rs           FormulaError
│   ├── formula.rs         the abstract syntax tree (AST) and the symbols
│   ├── parser.rs          RPN -> AST
│   ├── printer.rs         AST -> RPN
│   ├── evaluator.rs       ex 03
│   ├── truth_table.rs     ex 04 (computation separated from rendering)
│   ├── sat.rs             ex 07
│   ├── nnf.rs             ex 05
│   └── cnf.rs             ex 06
├── sets/                  ex 08-09
└── curve/                 ex 10-11
    ├── hilbert.rs         (x, y) <-> index along the curve
    ├── unit_interval.rs   index <-> real number in [0; 1]
    └── mapping.rs         map / reverse_map
tests/subject_examples.rs  the subject's examples
```

Principles:

- **One shared AST.** Exercises 03 to 09 all read formulas in reverse polish notation. `parse_rpn` turns them into a `Formula` once, and each exercise is a function over that tree.
- **One function = one responsibility.** Every computation step is a small, named function; the main functions only orchestrate.
- **Computation, logic and display are separate.** Only `main.rs` and `print_truth_table` write to standard output.

### Error handling

The subject leaves the behaviour on invalid input undefined, while encouraging an error message. Each subject function:

1. writes the error to **stderr**;
2. returns a neutral value: `false` (`eval_formula`, `sat`), an empty string (NNF, CNF), an empty list (`powerset`, `eval_set`) or `(0, 0)` (`reverse_map`).

The `try_*` variants (`try_eval_formula`, `try_sat`, `try_negation_normal_form`, `try_conjunctive_normal_form`, `try_eval_set`, `render_truth_table`) return a `Result<_, FormulaError>` so the caller can get the error.

### Design choices and limits

- **Ex 00 and 01:** only `& | ^ << >>`, comparisons and assignment; `adder` loops at most 32 times (O(log n)) and `multiplier` uses `adder` (32 iterations, O(1)). Overflow behaves like native `u32` (modulo 2³²).
- **Ex 04:** variables are sorted alphabetically, the first variable is the most significant bit; a formula can contain up to 26 distinct variables (2²⁶ rows).
- **Ex 05:** `^`, `>` and `=` are rewritten with `!`, `&`, `|`; negations are pushed down to the variables (double negation and De Morgan's laws).
- **Ex 06:** NNF, then distribution of `|` over `&`. Chains of `&` and `|` are nested to the right to match the subject's examples (`ABCD|||`). The output size may grow exponentially, which the subject allows.
- **Ex 08:** `powerset` rejects sets of more than 30 elements (2³⁰ subsets no longer fit in memory).
- **Ex 09:** the number of sets must equal the number of distinct variables; `A` is the first set, `B` the second, and so on. The complement is taken relative to the union of all the sets. The result is sorted and has no duplicates.
- **Ex 10 and 11 (optional):** order-16 Hilbert curve, bijective and continuous: two consecutive positions along the curve are always neighbouring cells. `map(0, 0) = 0.0` and `map(65535, 0) = 1.0`.

---

## Français

Implémentation en Rust des exercices 00 à 11 du sujet « Ready, Set, Boole! — An introduction to Boolean Algebra » (version 2.1).

Aucune dépendance externe : seule la bibliothèque standard est utilisée, et aucune fonction mathématique (`pow`, `sqrt`, ...) n'est appelée.

### Utilisation

```sh
cargo run      # démonstration de tous les exercices
cargo test     # tests unitaires (dans chaque module) + exemples du sujet (tests/)
cargo clippy --all-targets
```

### Fonctions du sujet

Elles sont toutes accessibles à la racine de la bibliothèque (`use ready_set_boole::*;`) avec les prototypes imposés.

| Ex | Prototype | Module |
|---|---|---|
| 00 | `fn adder(a: u32, b: u32) -> u32` | `arithmetic/adder.rs` |
| 01 | `fn multiplier(a: u32, b: u32) -> u32` | `arithmetic/multiplier.rs` |
| 02 | `fn gray_code(n: u32) -> u32` | `arithmetic/gray_code.rs` |
| 03 | `fn eval_formula(formula: &str) -> bool` | `logic/evaluator.rs` |
| 04 | `fn print_truth_table(formula: &str)` | `logic/truth_table.rs` |
| 05 | `fn negation_normal_form(formula: &str) -> String` | `logic/nnf.rs` |
| 06 | `fn conjunctive_normal_form(formula: &str) -> String` | `logic/cnf.rs` |
| 07 | `fn sat(formula: &str) -> bool` | `logic/sat.rs` |
| 08 | `fn powerset(set: Vec<i32>) -> Vec<Vec<i32>>` | `sets/powerset.rs` |
| 09 | `fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32>` | `sets/set_evaluator.rs` |
| 10 | `fn map(x: u16, y: u16) -> f64` | `curve/mapping.rs` |
| 11 | `fn reverse_map(n: f64) -> (u16, u16)` | `curve/mapping.rs` |

### Architecture

```
src/
├── lib.rs, main.rs        main.rs : démonstration (affichage uniquement)
├── arithmetic/            ex 00-02 : opérations sur les bits
├── logic/                 ex 03-07 : logique propositionnelle
│   ├── error.rs           FormulaError
│   ├── formula.rs         l'arbre syntaxique (AST) et les symboles
│   ├── parser.rs          RPN -> AST
│   ├── printer.rs         AST -> RPN
│   ├── evaluator.rs       ex 03
│   ├── truth_table.rs     ex 04 (calcul séparé du rendu)
│   ├── sat.rs             ex 07
│   ├── nnf.rs             ex 05
│   └── cnf.rs             ex 06
├── sets/                  ex 08-09
└── curve/                 ex 10-11
    ├── hilbert.rs         (x, y) <-> index le long de la courbe
    ├── unit_interval.rs   index <-> réel de [0; 1]
    └── mapping.rs         map / reverse_map
tests/subject_examples.rs  les exemples du sujet
```

Principes :

- **Un AST partagé.** Les exercices 03 à 09 lisent tous des formules en notation polonaise inverse. `parse_rpn` les transforme une seule fois en `Formula`, et chaque exercice est une fonction sur cet arbre.
- **Une fonction = une responsabilité.** Chaque étape de calcul est une petite fonction nommée ; les fonctions principales ne font qu'orchestrer.
- **Calcul, logique et affichage séparés.** Seuls `main.rs` et `print_truth_table` écrivent sur la sortie standard.

### Gestion des erreurs

Le sujet laisse indéfini le comportement sur entrée invalide, tout en encourageant un message d'erreur. Chaque fonction du sujet :

1. écrit l'erreur sur **stderr** ;
2. retourne une valeur neutre : `false` (`eval_formula`, `sat`), chaîne vide (NNF, CNF), liste vide (`powerset`, `eval_set`) ou `(0, 0)` (`reverse_map`).

Les versions `try_*` (`try_eval_formula`, `try_sat`, `try_negation_normal_form`, `try_conjunctive_normal_form`, `try_eval_set`, `render_truth_table`) retournent un `Result<_, FormulaError>` pour récupérer l'erreur.

### Choix et limites

- **Ex 00 et 01 :** uniquement `& | ^ << >>`, comparaisons et affectation ; `adder` boucle au plus 32 fois (O(log n)) et `multiplier` utilise `adder` (32 tours, O(1)). Le dépassement se comporte comme le `u32` natif (modulo 2³²).
- **Ex 04 :** variables triées par ordre alphabétique, première variable = bit de poids fort ; la formule peut contenir jusqu'à 26 variables distinctes (2²⁶ lignes).
- **Ex 05 :** `^`, `>` et `=` sont réécrits avec `!`, `&`, `|` ; les négations sont poussées vers les variables (double négation et lois de De Morgan).
- **Ex 06 :** NNF, puis distributivité de `|` sur `&`. Les chaînes de `&` et de `|` sont imbriquées à droite pour correspondre aux exemples du sujet (`ABCD|||`). La taille de sortie peut croître exponentiellement, ce que le sujet autorise.
- **Ex 08 :** `powerset` refuse plus de 30 éléments (2³⁰ parties ne tiennent plus en mémoire).
- **Ex 09 :** le nombre d'ensembles doit égaler le nombre de variables distinctes ; `A` désigne le premier ensemble, `B` le deuxième, etc. Le complément est pris par rapport à l'union de tous les ensembles. Le résultat est trié et sans doublon.
- **Ex 10 et 11 (optionnels) :** courbe de Hilbert d'ordre 16, bijective et continue : deux positions consécutives le long de la courbe sont toujours des cases voisines. `map(0, 0) = 0.0` et `map(65535, 0) = 1.0`.
