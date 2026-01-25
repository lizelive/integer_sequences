/// a(n) = n^3 + 2*n + 4
/// https://oeis.org/A000542

pub struct A000542;

impl crate::traits::IntegerSequence for A000542 {
    const NAME: &str = "a(n) = n^3 + 2*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 7, 16, 37, 76, 139, 232, 361, 532, 751, 1024, 1357, 1756, 2227, 2776, 3409, 4132, 4951, 5872, 6901, 8044, 9307, 10696, 12217, 13876, 15679, 17632, 19741, 22012, 24451
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000542";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_542(n)
    }
}

const fn poly_542(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000542>();
}
