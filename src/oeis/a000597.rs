/// a(n) = n^3 + 7*n + 9
/// https://oeis.org/A000597

pub struct A000597;

impl crate::traits::IntegerSequence for A000597 {
    const NAME: &str = "a(n) = n^3 + 7*n + 9";

    const HEAD: &[crate::Value] = &[
        9, 17, 31, 57, 101, 169, 267, 401, 577, 801, 1079, 1417, 1821, 2297, 2851, 3489, 4217, 5041, 5967, 7001, 8149, 9417, 10811, 12337, 14001, 15809, 17767, 19881, 22157, 24601
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000597";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_597(n)
    }
}

const fn poly_597(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 7 * n + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000597>();
}
