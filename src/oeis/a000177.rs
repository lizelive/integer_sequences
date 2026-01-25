/// a(n) = n^2 + 7*n + 2
/// https://oeis.org/A000177

pub struct A000177;

impl crate::traits::IntegerSequence for A000177 {
    const NAME: &str = "a(n) = n^2 + 7*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 10, 20, 32, 46, 62, 80, 100, 122, 146, 172, 200, 230, 262, 296, 332, 370, 410, 452, 496, 542, 590, 640, 692, 746
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000177";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_177(n)
    }
}

const fn poly_177(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 7 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000177>();
}
