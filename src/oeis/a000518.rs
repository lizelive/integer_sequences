/// a(n) = n^3 + 8*n + 1
/// https://oeis.org/A000518

pub struct A000518;

impl crate::traits::IntegerSequence for A000518 {
    const NAME: &str = "a(n) = n^3 + 8*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 10, 25, 52, 97, 166, 265, 400, 577, 802, 1081, 1420, 1825, 2302, 2857, 3496, 4225, 5050, 5977, 7012, 8161, 9430, 10825, 12352, 14017, 15826, 17785, 19900, 22177, 24622
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000518";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_518(n)
    }
}

const fn poly_518(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 8 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000518>();
}
