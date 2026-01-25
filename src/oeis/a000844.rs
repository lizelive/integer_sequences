/// a(n) = 5*T(n)^5
/// https://oeis.org/A000844

pub struct A000844;

impl crate::traits::IntegerSequence for A000844 {
    const NAME: &str = "a(n) = 5*T(n)^5";

    const HEAD: &[crate::Value] = &[
        0, 5, 1215, 38880, 500000, 3796875, 20420505, 86051840, 302330880, 922640625, 2516421875, 6261662880, 14435871840, 31201607255, 63814078125, 124416000000, 232629370880, 419205679965, 731055849255, 1238049500000, 2042050500000, 3288742750755, 5182897382465, 8007840506880, 12150000000000, 18129541015625, 26638243633755, 38585932791840, 55156940998880, 77878268859375
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000844";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_844(n)
    }
}

const fn tri_pow_844(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    5 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000844>();
}
