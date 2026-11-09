//! Exercice 01 : multiplication avec uniquement des opérations sur les bits.
//!
//! Principe (multiplication « à la main » en binaire) :
//!   a * b = somme, pour chaque bit i de b valant 1, de (a << i)
//! Exemple : 5 * 3 = 5*(0b11) = (5 << 0) + (5 << 1) = 5 + 10 = 15.
//! L'addition n'étant pas autorisée, on réutilise `adder`.
//!
//! Complexité : exactement 32 tours (taille du mot), donc O(1).
//! Mémoire : quelques variables de 32 bits, donc O(1).

use super::adder::adder;

/// Nombre de bits d'un `u32` : le nombre de tours de la boucle.
const WORD_BITS: u32 = 32;

/// Indique si le bit de poids faible de `n` vaut 1.
///
/// - Entrée : un entier.
/// - Opération : `n & 1` isole le bit de poids faible.
/// - Résultat : `true` s'il vaut 1. `multiplier` s'en sert pour savoir
///   s'il faut ajouter une copie décalée de `a`.
fn lowest_bit_is_set(n: u32) -> bool {
    let lowest_bit = n & 1;
    lowest_bit == 1
}

/// Retourne `a * b` (arithmétique modulo 2^32, comme le `u32` natif).
///
/// À chaque tour i, on regarde le bit i de `b` (en décalant `b` de i vers la droite).
/// S'il vaut 1, on ajoute `a << i` au total (poids 2^i de ce bit).
pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut product = 0;

    for bit_index in 0..WORD_BITS {
        let multiplier_shifted = b >> bit_index;
        if lowest_bit_is_set(multiplier_shifted) {
            let partial_product = a << bit_index;
            product = adder(product, partial_product);
        }
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplies_small_numbers() {
        assert_eq!(multiplier(5, 3), 15);
        assert_eq!(multiplier(7, 6), 42);
    }

    #[test]
    fn zero_annihilates_and_one_is_the_identity() {
        assert_eq!(multiplier(0, 123), 0);
        assert_eq!(multiplier(123, 0), 0);
        assert_eq!(multiplier(1, 123), 123);
        assert_eq!(multiplier(123, 1), 123);
    }

    #[test]
    fn powers_of_two_are_shifts() {
        assert_eq!(multiplier(13, 8), 13 << 3);
        assert_eq!(multiplier(1 << 15, 1 << 15), 1 << 30);
    }

    #[test]
    fn wraps_like_native_u32_on_overflow() {
        assert_eq!(multiplier(u32::MAX, 2), u32::MAX.wrapping_mul(2));
        assert_eq!(multiplier(u32::MAX, u32::MAX), 1);
        assert_eq!(multiplier(1 << 16, 1 << 16), 0);
    }

    #[test]
    fn matches_native_multiplication() {
        for a in [0u32, 1, 2, 3, 10, 255, 1000, 65_535, 1 << 31, u32::MAX] {
            for b in [0u32, 1, 7, 256, 99_999, u32::MAX] {
                assert_eq!(multiplier(a, b), a.wrapping_mul(b));
                assert_eq!(multiplier(a, b), multiplier(b, a));
            }
        }
    }

    #[test]
    fn lowest_bit_detection() {
        assert!(lowest_bit_is_set(1));
        assert!(lowest_bit_is_set(0b1011));
        assert!(!lowest_bit_is_set(0));
        assert!(!lowest_bit_is_set(0b1010));
    }
}
