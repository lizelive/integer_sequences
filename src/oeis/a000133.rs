/// a(n) = floor(n^4/4)
/// https://oeis.org/A000133

pub struct A000133;

impl crate::traits::IntegerSequence for A000133 {
    const NAME: &str = "a(n) = floor(n^4/4)";

    const HEAD: &[crate::Value] = &[
        0, 0, 4, 20, 64, 156, 324, 600, 1024, 1640, 2500, 3660, 5184, 7140, 9604, 12656, 16384, 20880, 26244, 32580, 40000, 48620, 58564, 69960, 82944, 97656, 114244, 132860, 153664, 176820
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000133";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        n_fourth_div_4(n)
    }
}

const fn n_fourth_div_4(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let n2 = n * n;
    n2 * n2 / 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000133>();
}
