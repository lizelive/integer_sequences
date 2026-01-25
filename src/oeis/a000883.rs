/// a(n) = 4*T(n)^4 + 1
/// https://oeis.org/A000883

pub struct A000883;

impl crate::traits::IntegerSequence for A000883 {
    const NAME: &str = "a(n) = 4*T(n)^4 + 1";

    const HEAD: &[crate::Value] = &[
        1, 5, 325, 5185, 40001, 202501, 777925, 2458625, 6718465, 16402501, 36602501, 75898945, 148060225, 274299845, 486202501, 829440001, 1368408065, 2191925125, 3420144325, 5212840001, 7779240001, 11389585285, 16388608325, 23211131905, 32400000001, 44626562501, 60713945605, 81663349825, 108683627585, 143224402501
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000883";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_883(n)
    }
}

const fn tri_pow_883(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    4 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000883>();
}
