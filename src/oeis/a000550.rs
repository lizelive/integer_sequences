/// a(n) = n^3 + 0*n + 5
/// https://oeis.org/A000550

pub struct A000550;

impl crate::traits::IntegerSequence for A000550 {
    const NAME: &str = "a(n) = n^3 + 0*n + 5";

    const HEAD: &[crate::Value] = &[
        5, 6, 13, 32, 69, 130, 221, 348, 517, 734, 1005, 1336, 1733, 2202, 2749, 3380, 4101, 4918, 5837, 6864, 8005, 9266, 10653, 12172, 13829, 15630, 17581, 19688, 21957, 24394
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000550";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_550(n)
    }
}

const fn poly_550(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000550>();
}
