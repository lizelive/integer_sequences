/// a(n) = 2*n^3 + 3*n^2 + 1*n
/// https://oeis.org/A001019

pub struct A001019;

impl crate::traits::IntegerSequence for A001019 {
    const NAME: &str = "a(n) = 2*n^3 + 3*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 6, 30, 84, 180, 330, 546, 840, 1224, 1710, 2310, 3036, 3900, 4914, 6090, 7440, 8976, 10710, 12654, 14820, 17220, 19866, 22770, 25944, 29400, 33150, 37206, 41580, 46284, 51330
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001019";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1019(n)
    }
}

const fn cubic_1019(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n * n + 3 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001019>();
}
