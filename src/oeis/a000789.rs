/// a(n) = 4*n^2 + 5*n + 2
/// https://oeis.org/A000789

pub struct A000789;

impl crate::traits::IntegerSequence for A000789 {
    const NAME: &str = "a(n) = 4*n^2 + 5*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 11, 28, 53, 86, 127, 176, 233, 298, 371, 452, 541, 638, 743, 856, 977, 1106, 1243, 1388, 1541, 1702, 1871, 2048, 2233, 2426, 2627, 2836, 3053, 3278, 3511
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000789";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_789(n)
    }
}

const fn quad_789(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 5 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000789>();
}
