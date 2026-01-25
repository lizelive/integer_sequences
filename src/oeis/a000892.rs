/// a(n) = 3*T(n)^5 + 1
/// https://oeis.org/A000892

pub struct A000892;

impl crate::traits::IntegerSequence for A000892 {
    const NAME: &str = "a(n) = 3*T(n)^5 + 1";

    const HEAD: &[crate::Value] = &[
        1, 4, 730, 23329, 300001, 2278126, 12252304, 51631105, 181398529, 553584376, 1509853126, 3756997729, 8661523105, 18720964354, 38288446876, 74649600001, 139577622529, 251523407980, 438633509554, 742829700001, 1225230300001, 1973245650454, 3109738429480, 4804704304129, 7290000000001, 10877724609376, 15982946180254, 23151559675105, 33094164599329, 46726961315626
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000892";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_892(n)
    }
}

const fn tri_pow_892(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    3 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000892>();
}
