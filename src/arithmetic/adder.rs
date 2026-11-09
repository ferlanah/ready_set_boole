//! Exercice 00 : addition de deux entiers avec uniquement des opérations sur les bits.
//!
//! Principe (l'additionneur binaire vu dans le sujet) :
//!   a + b = (a XOR b) + ((a AND b) << 1)
//! Le membre de droite est encore une addition, mais ses opérandes ont changé :
//! les retenues sont décalées vers la gauche à chaque tour, donc elles finissent
//! par sortir du mot de 32 bits et devenir nulles. On s'arrête à ce moment-là.
//!
//! Complexité : au plus 32 tours de boucle, soit O(log n) en la valeur des entrées.
//! Mémoire : 2 variables de 32 bits, soit O(1) (donc dans la borne O(log n)).

/// Additionne `a` et `b` bit à bit **sans propager la retenue**.
///
/// - Entrées : deux entiers.
/// - Opération : XOR. Pour chaque position, 0+0=0, 0+1=1, 1+0=1, 1+1=0
///   (le « 1+1 » perd sa retenue, c'est `carry_bits` qui la récupère).
/// - Résultat : la somme partielle, utilisée par `adder` comme nouveau `a`.
fn sum_without_carry(a: u32, b: u32) -> u32 {
    a ^ b
}

/// Calcule les retenues produites par l'addition de `a` et `b`,
/// déjà décalées à la position où elles doivent être ajoutées.
///
/// - Entrées : deux entiers.
/// - Opérations : une retenue naît là où les DEUX bits valent 1 (`a & b`),
///   et elle s'ajoute au bit de gauche (`<< 1`).
/// - Résultat : le mot de retenues, utilisé par `adder` comme nouveau `b`.
fn carry_bits(a: u32, b: u32) -> u32 {
    let positions_where_both_are_one = a & b;
    positions_where_both_are_one << 1
}

/// Retourne `a + b` (arithmétique modulo 2^32, comme le `u32` natif).
///
/// Orchestre la boucle : tant qu'il reste des retenues à ajouter,
/// on remplace (a, b) par (somme partielle, retenues).
pub fn adder(a: u32, b: u32) -> u32 {
    let mut partial_sum = a;
    let mut pending_carries = b;

    while pending_carries != 0 {
        let next_sum = sum_without_carry(partial_sum, pending_carries);
        let next_carries = carry_bits(partial_sum, pending_carries);
        partial_sum = next_sum;
        pending_carries = next_carries;
    }
    partial_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_small_numbers() {
        assert_eq!(adder(1, 2), 3);
        assert_eq!(adder(5, 7), 12);
    }

    #[test]
    fn zero_is_the_identity() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(42, 0), 42);
        assert_eq!(adder(0, 42), 42);
    }

    #[test]
    fn propagates_long_carry_chains() {
        // 0b0111_1111 + 1 : la retenue traverse 7 positions.
        assert_eq!(adder(127, 1), 128);
        assert_eq!(adder(u32::MAX - 1, 1), u32::MAX);
    }

    #[test]
    fn wraps_like_native_u32_on_overflow() {
        assert_eq!(adder(u32::MAX, 1), 0);
        assert_eq!(adder(u32::MAX, u32::MAX), u32::MAX - 1);
    }

    #[test]
    fn is_commutative_and_matches_native_addition() {
        for a in [0u32, 1, 2, 3, 255, 1000, 65_535, 1 << 31] {
            for b in [0u32, 1, 7, 256, 99_999, u32::MAX] {
                assert_eq!(adder(a, b), a.wrapping_add(b));
                assert_eq!(adder(a, b), adder(b, a));
            }
        }
    }

    #[test]
    fn helpers_have_a_single_clear_role() {
        assert_eq!(sum_without_carry(0b0110, 0b0011), 0b0101);
        assert_eq!(carry_bits(0b0110, 0b0011), 0b0100);
    }
}
