/// a(n) = 2^n + n^2
/// https://oeis.org/A000147

pub struct A000147;

impl crate::traits::IntegerSequence for A000147 {
    const NAME: &str = "a(n) = 2^n + n^2";

    const HEAD: &[crate::Value] = &[
        1, 3, 8, 17, 32, 57, 100, 177, 320, 593, 1124, 2169, 4240, 8361, 16580, 32993, 65792, 131361, 262468, 524649, 1048976, 2097593, 4194788, 8389137, 16777792
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000147";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power2_plus_n_sq(n)
    }
}

const fn power2_plus_n_sq(n: crate::Index) -> crate::Value {
    if n < 0 || n > 60 { return 0; }
    (1isize << n) + n * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000147>();
}
