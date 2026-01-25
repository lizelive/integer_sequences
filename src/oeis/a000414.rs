/// a(n) = 3*11^n
/// https://oeis.org/A000414

pub struct A000414;

impl crate::traits::IntegerSequence for A000414 {
    const NAME: &str = "a(n) = 3*11^n";

    const HEAD: &[crate::Value] = &[
        3, 33, 363, 3993, 43923, 483153, 5314683, 58461513, 643076643, 7073843073, 77812273803, 855935011833, 9415285130163, 103568136431793, 1139249500749723, 12531744508246953, 137849189590716483
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000414";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_414(n)
    }
}

const fn pow_414(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 11;
        i += 1;
    }
    3 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000414>();
}
