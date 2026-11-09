//! Courbe de Hilbert d'ordre 16 : correspondance entre un point (x, y) du carré
//! [0; 65535]^2 et sa position `index` le long de la courbe, dans [0; 2^32 - 1].
//!
//! Principe : on découpe le carré en 4 quadrants, on les visite dans un ordre qui
//! garde la courbe continue, puis on recommence DANS le quadrant, retourné (rotation
//! ou symétrie) pour que la sortie d'un quadrant touche l'entrée du suivant.
//! Chaque « niveau » (de 15 à 0) lit 1 bit de x et 1 bit de y, et produit un chiffre
//! de 2 bits de l'index. 16 niveaux × 2 bits = 32 bits d'index.
//!
//! Deux chaînes de calcul, qui sont l'inverse l'une de l'autre :
//!   point -> (pour chaque niveau : bits rx, ry -> chiffre -> rotation) -> index
//!   index -> (pour chaque niveau : chiffre -> bits rx, ry -> rotation -> bits ajoutés) -> point

/// Nombre de niveaux de subdivision : chaque coordonnée a 16 bits.
const LEVELS: u32 = 16;

/// Masque de tous les bits d'une coordonnée (65535).
const ALL_COORDINATE_BITS: u32 = 0xFFFF;

/// Un point en cours de transformation.
type Point = (u32, u32);

// ---------------------------------------------------------------------------
// Briques communes aux deux sens
// ---------------------------------------------------------------------------

/// Symétrie centrale dans le carré : chaque coordonnée devient « son miroir ».
/// XOR avec un masque de 1 inverse tous les bits concernés (équivaut à `masque - valeur`).
fn flip_both_coordinates(point: Point, mask: u32) -> Point {
    let (x, y) = point;
    (x ^ mask, y ^ mask)
}

/// Échange x et y (symétrie par rapport à la diagonale).
fn swap_axes(point: Point) -> Point {
    let (x, y) = point;
    (y, x)
}

/// Retourne le point pour l'orienter comme le sous-carré dans lequel on entre.
///
/// - `ry == 1` : le quadrant est dans la même orientation, rien à faire.
/// - `ry == 0` : on échange x et y, et, si `rx == 1`, on commence par une symétrie centrale.
///
/// `flip_mask` donne la taille du carré dans lequel la symétrie s'applique.
fn rotate_quadrant(point: Point, rx: u32, ry: u32, flip_mask: u32) -> Point {
    if ry == 1 {
        return point;
    }
    let point = if rx == 1 {
        flip_both_coordinates(point, flip_mask)
    } else {
        point
    };
    swap_axes(point)
}

// ---------------------------------------------------------------------------
// Sens direct : point -> index
// ---------------------------------------------------------------------------

/// Bit numéro `level` d'une coordonnée (0 ou 1).
fn bit_at(coordinate: u32, level: u32) -> u32 {
    (coordinate >> level) & 1
}

/// Rang (0 à 3) du quadrant désigné par les bits (rx, ry), dans l'ordre de visite
/// de Hilbert : (0,0) -> 0, (0,1) -> 1, (1,1) -> 2, (1,0) -> 3.
fn quadrant_digit(rx: u32, ry: u32) -> u32 {
    let three_times_rx = 3 * rx;
    three_times_rx ^ ry
}

/// Place un chiffre de 2 bits à la position du niveau `level` dans l'index.
fn place_digit(digit: u32, level: u32) -> u32 {
    let shift = 2 * level;
    digit << shift
}

/// Position le long de la courbe du point (x, y).
///
/// On part du niveau le plus grossier (bit 15) vers le plus fin (bit 0) :
/// 1. lire les bits rx, ry ;
/// 2. en déduire le chiffre du quadrant et l'ajouter à l'index ;
/// 3. retourner le point pour traiter le niveau suivant.
pub fn xy_to_index(x: u16, y: u16) -> u32 {
    let mut point: Point = (x as u32, y as u32);
    let mut index = 0;

    for level in (0..LEVELS).rev() {
        let rx = bit_at(point.0, level);
        let ry = bit_at(point.1, level);

        index |= place_digit(quadrant_digit(rx, ry), level);
        point = rotate_quadrant(point, rx, ry, ALL_COORDINATE_BITS);
    }
    index
}

// ---------------------------------------------------------------------------
// Sens inverse : index -> point
// ---------------------------------------------------------------------------

/// Chiffre de 2 bits (0 à 3) de l'index au niveau `level` : l'inverse de `place_digit`.
fn extract_digit(index: u32, level: u32) -> u32 {
    let shift = 2 * level;
    (index >> shift) & 0b11
}

/// Retrouve les bits (rx, ry) à partir du rang du quadrant : l'inverse de `quadrant_digit`.
fn quadrant_bits(digit: u32) -> (u32, u32) {
    let rx = (digit >> 1) & 1;
    let ry = (digit ^ rx) & 1;
    (rx, ry)
}

/// Masque de tous les bits situés sous le niveau `level` (ceux déjà reconstruits).
/// Exemple : niveau 3 -> 0b111.
fn lower_bits_mask(level: u32) -> u32 {
    (1 << level) - 1
}

/// Ajoute rx et ry comme bits de poids `level` des coordonnées reconstruites.
fn set_quadrant_bits(point: Point, rx: u32, ry: u32, level: u32) -> Point {
    let (x, y) = point;
    (x | (rx << level), y | (ry << level))
}

/// Point (x, y) situé à la position `index` de la courbe.
///
/// On part du niveau le plus fin (chiffre de poids faible) vers le plus grossier,
/// en reconstruisant les bits des coordonnées du bas vers le haut.
pub fn index_to_xy(index: u32) -> (u16, u16) {
    let mut point: Point = (0, 0);

    for level in 0..LEVELS {
        let digit = extract_digit(index, level);
        let (rx, ry) = quadrant_bits(digit);

        point = rotate_quadrant(point, rx, ry, lower_bits_mask(level));
        point = set_quadrant_bits(point, rx, ry, level);
    }
    (point.0 as u16, point.1 as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Distance de Manhattan entre deux points (nombre de pas horizontaux + verticaux).
    fn manhattan_distance(a: (u16, u16), b: (u16, u16)) -> u32 {
        a.0.abs_diff(b.0) as u32 + a.1.abs_diff(b.1) as u32
    }

    #[test]
    fn building_blocks() {
        assert_eq!(bit_at(0b1010, 1), 1);
        assert_eq!(bit_at(0b1010, 2), 0);
        assert_eq!(quadrant_digit(0, 0), 0);
        assert_eq!(quadrant_digit(0, 1), 1);
        assert_eq!(quadrant_digit(1, 1), 2);
        assert_eq!(quadrant_digit(1, 0), 3);
        assert_eq!(place_digit(0b11, 2), 0b11_0000);
        assert_eq!(lower_bits_mask(0), 0);
        assert_eq!(lower_bits_mask(3), 0b111);
        assert_eq!(swap_axes((1, 2)), (2, 1));
        assert_eq!(flip_both_coordinates((0b01, 0b10), 0b11), (0b10, 0b01));
    }

    #[test]
    fn quadrant_digit_and_bits_are_inverse() {
        for digit in 0..4 {
            let (rx, ry) = quadrant_bits(digit);
            assert_eq!(quadrant_digit(rx, ry), digit);
        }
    }

    #[test]
    fn extract_digit_is_the_inverse_of_place_digit() {
        for level in 0..LEVELS {
            for digit in 0..4 {
                assert_eq!(extract_digit(place_digit(digit, level), level), digit);
            }
        }
    }

    #[test]
    fn rotation_leaves_quadrant_with_ry_one_unchanged() {
        assert_eq!(rotate_quadrant((5, 9), 0, 1, 0xF), (5, 9));
        assert_eq!(rotate_quadrant((5, 9), 1, 1, 0xF), (5, 9));
    }

    #[test]
    fn rotation_with_ry_zero_swaps_and_maybe_flips() {
        assert_eq!(rotate_quadrant((1, 2), 0, 0, 0xF), (2, 1));
        assert_eq!(rotate_quadrant((1, 2), 1, 0, 0xF), (0xF ^ 2, 0xF ^ 1));
    }

    #[test]
    fn curve_starts_at_origin_and_ends_on_the_bottom_right_corner() {
        assert_eq!(xy_to_index(0, 0), 0);
        assert_eq!(index_to_xy(0), (0, 0));
        assert_eq!(index_to_xy(u32::MAX), (u16::MAX, 0));
        assert_eq!(xy_to_index(u16::MAX, 0), u32::MAX);
    }

    #[test]
    fn index_to_xy_inverts_xy_to_index_on_axes_and_diagonals() {
        for x in 0..=u16::MAX {
            for (a, b) in [(x, 0), (0, x), (x, x), (x, u16::MAX - x)] {
                assert_eq!(index_to_xy(xy_to_index(a, b)), (a, b));
            }
        }
    }

    #[test]
    fn index_to_xy_inverts_xy_to_index_on_a_wide_grid() {
        for x in (0..=u16::MAX).step_by(257) {
            for y in (0..=u16::MAX).step_by(251) {
                assert_eq!(index_to_xy(xy_to_index(x, y)), (x, y));
            }
        }
    }

    #[test]
    fn xy_to_index_inverts_index_to_xy_on_sampled_indexes() {
        for index in (0..=u32::MAX).step_by(999_983) {
            let (x, y) = index_to_xy(index);
            assert_eq!(xy_to_index(x, y), index);
        }
        assert_eq!(xy_to_index(index_to_xy(u32::MAX).0, index_to_xy(u32::MAX).1), u32::MAX);
    }

    #[test]
    fn is_bijective_on_a_dense_block() {
        let mut seen = std::collections::BTreeSet::new();
        for x in 0..256u16 {
            for y in 0..256u16 {
                assert!(seen.insert(xy_to_index(x, y)), "collision en ({x}, {y})");
            }
        }
        assert_eq!(seen.len(), 65_536);
    }

    #[test]
    fn curve_is_continuous_at_the_start() {
        for index in 0..300_000u32 {
            let distance = manhattan_distance(index_to_xy(index), index_to_xy(index + 1));
            assert_eq!(distance, 1, "entre les index {index} et {}", index + 1);
        }
    }

    #[test]
    fn curve_is_continuous_everywhere_on_samples() {
        for start in (0..u32::MAX - 100).step_by(1_000_003) {
            for index in start..start + 100 {
                let distance = manhattan_distance(index_to_xy(index), index_to_xy(index + 1));
                assert_eq!(distance, 1, "entre les index {index} et {}", index + 1);
            }
        }
    }

    #[test]
    fn continuity_across_the_four_main_quadrant_borders() {
        for border in [1u32 << 30, 2 << 30, 3 << 30] {
            let distance = manhattan_distance(index_to_xy(border - 1), index_to_xy(border));
            assert_eq!(distance, 1, "frontière {border}");
        }
    }
}
