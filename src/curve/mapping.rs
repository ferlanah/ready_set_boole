//! Exercices 10 et 11 : les deux fonctions imposées par le sujet.
//!
//! map         : (x, y) --xy_to_index--> index --index_to_unit_interval--> réel de [0; 1]
//! reverse_map : réel --unit_interval_to_index--> index --index_to_xy--> (x, y)

use super::hilbert::{index_to_xy, xy_to_index};
use super::unit_interval::{index_to_unit_interval, is_in_unit_interval, unit_interval_to_index};

/// Prototype imposé (ex 10) : associe à (x, y) un réel unique de [0; 1].
pub fn map(x: u16, y: u16) -> f64 {
    let index = xy_to_index(x, y);
    index_to_unit_interval(index)
}

/// Prototype imposé (ex 11) : l'inverse de `map`.
///
/// Hors de [0; 1] (ou NaN) : message sur stderr et retourne `(0, 0)`.
pub fn reverse_map(n: f64) -> (u16, u16) {
    if !is_in_unit_interval(n) {
        eprintln!("Erreur : {n} est hors de l'intervalle [0; 1]");
        return (0, 0);
    }
    let index = unit_interval_to_index(n);
    index_to_xy(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corners_map_to_interval_bounds() {
        assert_eq!(map(0, 0), 0.0);
        assert_eq!(map(u16::MAX, 0), 1.0); // fin de la courbe de Hilbert
        assert_eq!(reverse_map(0.0), (0, 0));
        assert_eq!(reverse_map(1.0), (u16::MAX, 0));
    }

    #[test]
    fn map_stays_in_unit_interval() {
        for (x, y) in [(0, 1), (1, 0), (65_535, 65_535), (0, 65_535), (123, 45_678)] {
            let value = map(x, y);
            assert!(is_in_unit_interval(value), "({x}, {y}) -> {value}");
        }
    }

    #[test]
    fn reverse_map_inverts_map_on_axes_and_diagonals() {
        for x in 0..=u16::MAX {
            for (a, b) in [(x, 0), (0, x), (x, x), (x, u16::MAX - x)] {
                assert_eq!(reverse_map(map(a, b)), (a, b));
            }
        }
    }

    #[test]
    fn reverse_map_inverts_map_on_a_wide_grid() {
        for x in (0..=u16::MAX).step_by(257) {
            for y in (0..=u16::MAX).step_by(251) {
                assert_eq!(reverse_map(map(x, y)), (x, y));
            }
        }
    }

    #[test]
    fn map_inverts_reverse_map_on_values_it_produced() {
        for index in (0..=u32::MAX).step_by(1_000_003) {
            let value = index_to_unit_interval(index);
            let (x, y) = reverse_map(value);
            assert_eq!(map(x, y), value);
        }
    }

    #[test]
    fn map_is_injective_on_a_dense_block() {
        let mut seen = std::collections::BTreeSet::new();
        for x in 0..256u16 {
            for y in 0..256u16 {
                assert!(seen.insert(map(x, y).to_bits()), "collision en ({x}, {y})");
            }
        }
        assert_eq!(seen.len(), 65_536);
    }

    #[test]
    fn neighbouring_values_stay_close_in_space() {
        // Propriété de la courbe continue : deux positions consécutives sont voisines.
        let value = map(1000, 2000);
        let next = value + 1.0 / u32::MAX as f64;
        let (x, y) = reverse_map(next);
        assert_eq!(1000u16.abs_diff(x) + 2000u16.abs_diff(y), 1);
    }

    #[test]
    fn out_of_range_input_returns_origin() {
        assert_eq!(reverse_map(-0.1), (0, 0));
        assert_eq!(reverse_map(1.5), (0, 0));
        assert_eq!(reverse_map(f64::NAN), (0, 0));
    }
}
