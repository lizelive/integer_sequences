/// a(n) = 5*n^3
/// https://oeis.org/A000264

pub struct A000264;

impl crate::traits::IntegerSequence for A000264 {
    const NAME: &str = "a(n) = 5*n^3";

    const HEAD: &[crate::Value] = &[
        0, 5, 40, 135, 320, 625, 1080, 1715, 2560, 3645, 5000, 6655, 8640, 10985, 13720, 16875, 20480, 24565, 29160, 34295, 40000, 46305, 53240, 60835, 69120
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000264";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_264(n)
    }
}

const fn power_264(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 3 {
        result *= n;
        i += 1;
    }
    5 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000264>();
}
