/// a(n) = 3*n^3 + 0*n^2 + 2*n
/// https://oeis.org/A001032

pub struct A001032;

impl crate::traits::IntegerSequence for A001032 {
    const NAME: &str = "a(n) = 3*n^3 + 0*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 5, 28, 87, 200, 385, 660, 1043, 1552, 2205, 3020, 4015, 5208, 6617, 8260, 10155, 12320, 14773, 17532, 20615, 24040, 27825, 31988, 36547, 41520, 46925, 52780, 59103, 65912, 73225
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001032";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1032(n)
    }
}

const fn cubic_1032(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n * n + 0 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001032>();
}
