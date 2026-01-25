/// a(n) = 9*n^2 + 9
/// https://oeis.org/A000998

pub struct A000998;

impl crate::traits::IntegerSequence for A000998 {
    const NAME: &str = "a(n) = 9*n^2 + 9";

    const HEAD: &[crate::Value] = &[
        9, 18, 45, 90, 153, 234, 333, 450, 585, 738, 909, 1098, 1305, 1530, 1773, 2034, 2313, 2610, 2925, 3258, 3609, 3978, 4365, 4770, 5193, 5634, 6093, 6570, 7065, 7578
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000998";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_998(n)
    }
}

const fn sq_998(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    9 * n * n + 9 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000998>();
}
