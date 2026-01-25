/// a(n) = 10*11^n
/// https://oeis.org/A000449

pub struct A000449;

impl crate::traits::IntegerSequence for A000449 {
    const NAME: &str = "a(n) = 10*11^n";

    const HEAD: &[crate::Value] = &[
        10, 110, 1210, 13310, 146410, 1610510, 17715610, 194871710, 2143588810, 23579476910, 259374246010, 2853116706110, 31384283767210, 345227121439310, 3797498335832410, 41772481694156510, 459497298635721610
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000449";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_449(n)
    }
}

const fn pow_449(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 11;
        i += 1;
    }
    10 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000449>();
}
