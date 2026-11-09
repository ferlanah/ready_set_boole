//! Conversion entre un index entier de la courbe (0 à 2^32 - 1) et un réel de [0; 1].
//!
//! Aller : index --diviser par le plus grand index--> réel.
//! Retour : réel --multiplier par le plus grand index--> flottant --arrondir--> index.
//!
//! Les 2^32 valeurs tiennent largement dans les 53 bits de mantisse d'un `f64` :
//! deux index distincts donnent deux réels distincts, et l'erreur d'arrondi du
//! retour (inférieure à 10^-6) est très inférieure à 0.5, donc l'arrondi est exact.

/// Plus grand index possible (2^32 - 1), en flottant.
const MAX_INDEX: f64 = u32::MAX as f64;

/// Indique si `value` appartient à [0; 1] (faux pour NaN).
pub fn is_in_unit_interval(value: f64) -> bool {
    (0.0..=1.0).contains(&value)
}

/// Index -> réel de [0; 1].
pub fn index_to_unit_interval(index: u32) -> f64 {
    let index_as_float = index as f64;
    index_as_float / MAX_INDEX
}

/// Étale un réel de [0; 1] sur l'échelle des index : [0; 2^32 - 1].
fn scale_to_index_range(value: f64) -> f64 {
    value * MAX_INDEX
}

/// Arrondit au plus proche. `+ 0.5` puis la conversion `as` qui tronque : 2.6 -> 3.1 -> 3.
fn round_to_nearest(value: f64) -> u32 {
    let shifted_by_half = value + 0.5;
    shifted_by_half as u32
}

/// Réel de [0; 1] -> index le plus proche.
pub fn unit_interval_to_index(value: f64) -> u32 {
    let scaled = scale_to_index_range(value);
    round_to_nearest(scaled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_of_the_interval() {
        assert_eq!(index_to_unit_interval(0), 0.0);
        assert_eq!(index_to_unit_interval(u32::MAX), 1.0);
        assert_eq!(unit_interval_to_index(0.0), 0);
        assert_eq!(unit_interval_to_index(1.0), u32::MAX);
    }

    #[test]
    fn range_check() {
        assert!(is_in_unit_interval(0.0));
        assert!(is_in_unit_interval(0.5));
        assert!(is_in_unit_interval(1.0));
        assert!(!is_in_unit_interval(-0.000_001));
        assert!(!is_in_unit_interval(1.000_001));
        assert!(!is_in_unit_interval(f64::NAN));
        assert!(!is_in_unit_interval(f64::INFINITY));
    }

    #[test]
    fn rounding_goes_to_the_nearest_integer() {
        assert_eq!(round_to_nearest(2.4), 2);
        assert_eq!(round_to_nearest(2.5), 3);
        assert_eq!(round_to_nearest(2.6), 3);
    }

    #[test]
    fn round_trip_is_exact_on_samples() {
        for index in (0..=u32::MAX).step_by(65_537) {
            assert_eq!(unit_interval_to_index(index_to_unit_interval(index)), index);
        }
        for index in 0..100_000u32 {
            assert_eq!(unit_interval_to_index(index_to_unit_interval(index)), index);
            let top = u32::MAX - index;
            assert_eq!(unit_interval_to_index(index_to_unit_interval(top)), top);
        }
    }

    #[test]
    fn conversion_is_strictly_increasing_hence_injective() {
        let mut previous = index_to_unit_interval(0);
        for index in (1..=100_000u32).chain(u32::MAX - 100_000..=u32::MAX) {
            let current = index_to_unit_interval(index);
            assert!(current > previous, "index {index}");
            previous = current;
        }
    }
}
