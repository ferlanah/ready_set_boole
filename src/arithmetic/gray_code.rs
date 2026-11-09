//! Exercice 02 : conversion d'un entier en code de Gray.
//!
//! Le code de Gray est tel que deux entiers consécutifs diffèrent d'un seul bit.
//! Formule : chaque bit du résultat est le XOR du bit correspondant de `n`
//! et du bit juste à sa gauche, ce qui s'écrit `n ^ (n >> 1)`.

/// Retourne le code de Gray de `n`.
///
/// - Entrée : un entier naturel.
/// - Opérations : on décale `n` d'un cran vers la droite (chaque bit se retrouve
///   aligné avec son voisin de gauche), puis on fait un XOR avec `n`.
/// - Résultat : le code de Gray (0→0, 1→1, 2→3, 3→2, 4→6, ...).
pub fn gray_code(n: u32) -> u32 {
    let n_shifted_right = n >> 1;
    n ^ n_shifted_right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_subject_examples() {
        let expected = [0, 1, 3, 2, 6, 7, 5, 4, 12];
        for (n, gray) in expected.iter().enumerate() {
            assert_eq!(gray_code(n as u32), *gray, "gray_code({n})");
        }
    }

    #[test]
    fn consecutive_values_differ_by_exactly_one_bit() {
        for n in 0..10_000u32 {
            let bits_changed = (gray_code(n) ^ gray_code(n + 1)).count_ones();
            assert_eq!(bits_changed, 1, "entre {n} et {}", n + 1);
        }
    }

    #[test]
    fn handles_extreme_values() {
        assert_eq!(gray_code(0), 0);
        assert_eq!(gray_code(u32::MAX), 1 << 31);
        assert_eq!(gray_code(1 << 31), (1 << 31) | (1 << 30));
    }
}
