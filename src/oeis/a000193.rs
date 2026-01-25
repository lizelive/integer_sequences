/// a(n) = n^2 + 3*n + 4
/// https://oeis.org/A000193

pub struct A000193;

impl crate::traits::IntegerSequence for A000193 {
    const NAME: &str = "a(n) = n^2 + 3*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 8, 14, 22, 32, 44, 58, 74, 92, 112, 134, 158, 184, 212, 242, 274, 308, 344, 382, 422, 464, 508, 554, 602, 652
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000193";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_193(n)
    }
}

const fn poly_193(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 3 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000193>();
}
