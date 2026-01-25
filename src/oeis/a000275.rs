/// a(n) = 6*n^4
/// https://oeis.org/A000275

pub struct A000275;

impl crate::traits::IntegerSequence for A000275 {
    const NAME: &str = "a(n) = 6*n^4";

    const HEAD: &[crate::Value] = &[
        0, 6, 96, 486, 1536, 3750, 7776, 14406, 24576, 39366, 60000, 87846, 124416, 171366, 230496, 303750, 393216, 501126, 629856, 781926, 960000, 1166886, 1405536, 1679046, 1990656
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000275";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_275(n)
    }
}

const fn power_275(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 4 {
        result *= n;
        i += 1;
    }
    6 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000275>();
}
