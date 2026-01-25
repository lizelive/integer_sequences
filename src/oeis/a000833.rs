/// a(n) = 4*T(n)^4
/// https://oeis.org/A000833

pub struct A000833;

impl crate::traits::IntegerSequence for A000833 {
    const NAME: &str = "a(n) = 4*T(n)^4";

    const HEAD: &[crate::Value] = &[
        0, 4, 324, 5184, 40000, 202500, 777924, 2458624, 6718464, 16402500, 36602500, 75898944, 148060224, 274299844, 486202500, 829440000, 1368408064, 2191925124, 3420144324, 5212840000, 7779240000, 11389585284, 16388608324, 23211131904, 32400000000, 44626562500, 60713945604, 81663349824, 108683627584, 143224402500
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000833";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_833(n)
    }
}

const fn tri_pow_833(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    4 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000833>();
}
