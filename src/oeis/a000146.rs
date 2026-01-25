/// a(n) = 2^n + n
/// https://oeis.org/A000146

pub struct A000146;

impl crate::traits::IntegerSequence for A000146 {
    const NAME: &str = "a(n) = 2^n + n";

    const HEAD: &[crate::Value] = &[
        1, 3, 6, 11, 20, 37, 70, 135, 264, 521, 1034, 2059, 4108, 8205, 16398, 32783, 65552, 131089, 262162, 524307, 1048596, 2097173, 4194326, 8388631, 16777240
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000146";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power2_plus_n(n)
    }
}

const fn power2_plus_n(n: crate::Index) -> crate::Value {
    if n < 0 || n > 60 { return 0; }
    (1isize << n) + n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000146>();
}
