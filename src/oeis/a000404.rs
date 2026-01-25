/// a(n) = 1*11^n
/// https://oeis.org/A000404

pub struct A000404;

impl crate::traits::IntegerSequence for A000404 {
    const NAME: &str = "a(n) = 1*11^n";

    const HEAD: &[crate::Value] = &[
        1, 11, 121, 1331, 14641, 161051, 1771561, 19487171, 214358881, 2357947691, 25937424601, 285311670611, 3138428376721, 34522712143931, 379749833583241, 4177248169415651, 45949729863572161, 505447028499293771
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000404";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_404(n)
    }
}

const fn pow_404(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 11;
        i += 1;
    }
    1 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000404>();
}
