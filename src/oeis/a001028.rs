/// a(n) = 5*n^3 + 4*n^2 + 1*n
/// https://oeis.org/A001028

pub struct A001028;

impl crate::traits::IntegerSequence for A001028 {
    const NAME: &str = "a(n) = 5*n^3 + 4*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 10, 58, 174, 388, 730, 1230, 1918, 2824, 3978, 5410, 7150, 9228, 11674, 14518, 17790, 21520, 25738, 30474, 35758, 41620, 48090, 55198, 62974, 71448, 80650, 90610, 101358, 112924, 125338
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001028";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1028(n)
    }
}

const fn cubic_1028(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n * n + 4 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001028>();
}
