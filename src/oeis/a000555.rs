/// a(n) = n^3 + 5*n + 5
/// https://oeis.org/A000555

pub struct A000555;

impl crate::traits::IntegerSequence for A000555 {
    const NAME: &str = "a(n) = n^3 + 5*n + 5";

    const HEAD: &[crate::Value] = &[
        5, 11, 23, 47, 89, 155, 251, 383, 557, 779, 1055, 1391, 1793, 2267, 2819, 3455, 4181, 5003, 5927, 6959, 8105, 9371, 10763, 12287, 13949, 15755, 17711, 19823, 22097, 24539
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000555";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_555(n)
    }
}

const fn poly_555(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 5 * n + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000555>();
}
