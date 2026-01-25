/// a(n) = n^3 + 3*n + 6
/// https://oeis.org/A000563

pub struct A000563;

impl crate::traits::IntegerSequence for A000563 {
    const NAME: &str = "a(n) = n^3 + 3*n + 6";

    const HEAD: &[crate::Value] = &[
        6, 10, 20, 42, 82, 146, 240, 370, 542, 762, 1036, 1370, 1770, 2242, 2792, 3426, 4150, 4970, 5892, 6922, 8066, 9330, 10720, 12242, 13902, 15706, 17660, 19770, 22042, 24482
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000563";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_563(n)
    }
}

const fn poly_563(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000563>();
}
