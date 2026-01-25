/// a(n) = 7*2^n
/// https://oeis.org/A000430

pub struct A000430;

impl crate::traits::IntegerSequence for A000430 {
    const NAME: &str = "a(n) = 7*2^n";

    const HEAD: &[crate::Value] = &[
        7, 14, 28, 56, 112, 224, 448, 896, 1792, 3584, 7168, 14336, 28672, 57344, 114688, 229376, 458752, 917504, 1835008, 3670016, 7340032, 14680064, 29360128, 58720256, 117440512
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000430";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_430(n)
    }
}

const fn pow_430(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 2;
        i += 1;
    }
    7 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000430>();
}
