/// a(n) = 3*T(n)^5
/// https://oeis.org/A000842

pub struct A000842;

impl crate::traits::IntegerSequence for A000842 {
    const NAME: &str = "a(n) = 3*T(n)^5";

    const HEAD: &[crate::Value] = &[
        0, 3, 729, 23328, 300000, 2278125, 12252303, 51631104, 181398528, 553584375, 1509853125, 3756997728, 8661523104, 18720964353, 38288446875, 74649600000, 139577622528, 251523407979, 438633509553, 742829700000, 1225230300000, 1973245650453, 3109738429479, 4804704304128, 7290000000000, 10877724609375, 15982946180253, 23151559675104, 33094164599328, 46726961315625
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000842";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_842(n)
    }
}

const fn tri_pow_842(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    3 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000842>();
}
