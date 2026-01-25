/// Number of orbits for the rotation group of a cube acting on the N-tuples.
/// https://oeis.org/A000066

pub struct A000066;

impl crate::traits::IntegerSequence for A000066 {
    const NAME: &str = "Number of orbits of rotation group of cube acting on N-tuples";

    const HEAD: &[crate::Value] = &[
        1, 1, 8, 57, 400, 2865, 20832, 153217, 1133104, 8418753, 62727600, 468248313, 3501337216,
        26217818673, 196482553456, 1473621397185, 11061655768288, 83059478915505,
        623969610428352, 4689523193652601,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000066";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        0
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000066>();
}
