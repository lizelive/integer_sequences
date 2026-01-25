/// a(n) = 4*n^3 + 3*n^2 + 2*n
/// https://oeis.org/A001051

pub struct A001051;

impl crate::traits::IntegerSequence for A001051 {
    const NAME: &str = "a(n) = 4*n^3 + 3*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 9, 48, 141, 312, 585, 984, 1533, 2256, 3177, 4320, 5709, 7368, 9321, 11592, 14205, 17184, 20553, 24336, 28557, 33240, 38409, 44088, 50301, 57072, 64425, 72384, 80973, 90216, 100137
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001051";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1051(n)
    }
}

const fn cubic_1051(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n * n + 3 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001051>();
}
