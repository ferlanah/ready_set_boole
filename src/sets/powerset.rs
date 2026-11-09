//! Exercice 08 : ensemble des parties.
//!
//! Idée : un sous-ensemble d'un ensemble à `n` éléments se décrit par un « masque »
//! de `n` bits (bit i à 1 = l'élément i est choisi). Les entiers de 0 à 2^n - 1
//! énumèrent donc exactement les 2^n sous-ensembles, sans doublon.
//! Mémoire : O(2^n), imposée par la taille du résultat.

/// Au-delà, le résultat (2^n parties) ne tient de toute façon plus en mémoire.
const MAX_SET_SIZE: usize = 30;

/// Construit le sous-ensemble désigné par `mask` : garde `set[i]` si le bit i vaut 1.
fn subset_from_mask(set: &[i32], mask: u32) -> Vec<i32> {
    let mut subset = Vec::new();
    for (index, &element) in set.iter().enumerate() {
        let element_is_chosen = (mask >> index) & 1 == 1;
        if element_is_chosen {
            subset.push(element);
        }
    }
    subset
}

/// Prototype imposé par le sujet.
///
/// Masque 0 donne l'ensemble vide, et le masque 2^n - 1 donne l'ensemble complet.
/// Si l'ensemble est trop grand (plus de 30 éléments), écrit l'erreur sur stderr
/// et retourne une liste vide.
pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    if set.len() > MAX_SET_SIZE {
        eprintln!(
            "Erreur : ensemble trop grand ({} éléments, maximum {MAX_SET_SIZE})",
            set.len()
        );
        return Vec::new();
    }

    let subset_count: u32 = 1 << set.len();
    (0..subset_count)
        .map(|mask| subset_from_mask(&set, mask))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut parts: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        parts.iter_mut().for_each(|part| part.sort());
        parts.sort();
        parts
    }

    #[test]
    fn empty_set_has_only_the_empty_set() {
        assert_eq!(powerset(vec![]), vec![Vec::<i32>::new()]);
    }

    #[test]
    fn single_element() {
        assert_eq!(sorted(powerset(vec![7])), vec![vec![], vec![7]]);
    }

    #[test]
    fn three_elements_give_eight_distinct_parts() {
        let expected = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 3],
            vec![2],
            vec![2, 3],
            vec![3],
        ];
        assert_eq!(sorted(powerset(vec![1, 2, 3])), expected);
    }

    #[test]
    fn size_is_two_to_the_n_without_duplicates() {
        for n in 0..=12usize {
            let set: Vec<i32> = (0..n as i32).collect();
            let parts = sorted(powerset(set));
            assert_eq!(parts.len(), 1 << n);
            let mut deduplicated = parts.clone();
            deduplicated.dedup();
            assert_eq!(deduplicated.len(), parts.len());
        }
    }

    #[test]
    fn handles_negative_and_zero_elements() {
        let parts = sorted(powerset(vec![-1, 0]));
        assert_eq!(parts, vec![vec![], vec![-1], vec![-1, 0], vec![0]]);
    }

    #[test]
    fn too_large_input_returns_empty_list() {
        assert!(powerset((0..31).collect()).is_empty());
    }

    #[test]
    fn mask_selects_the_right_elements() {
        assert_eq!(subset_from_mask(&[10, 20, 30], 0b101), vec![10, 30]);
        assert_eq!(subset_from_mask(&[10, 20, 30], 0), Vec::<i32>::new());
    }
}
