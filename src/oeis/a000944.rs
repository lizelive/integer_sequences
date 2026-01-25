/// a(n) = 5*n^2 + 4
/// https://oeis.org/A000944

pub struct A000944;

impl crate::traits::IntegerSequence for A000944 {
    const NAME: &str = "a(n) = 5*n^2 + 4";

    const HEAD: &[crate::Value] = &[
        4, 9, 24, 49, 84, 129, 184, 249, 324, 409, 504, 609, 724, 849, 984, 1129, 1284, 1449, 1624, 1809, 2004, 2209, 2424, 2649, 2884, 3129, 3384, 3649, 3924, 4209
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000944";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_944(n)
    }
}

const fn sq_944(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n + 4 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000944>();
}
