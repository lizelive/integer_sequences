/// a(n) = 3*n^5
/// https://oeis.org/A000282

pub struct A000282;

impl crate::traits::IntegerSequence for A000282 {
    const NAME: &str = "a(n) = 3*n^5";

    const HEAD: &[crate::Value] = &[
        0, 3, 96, 729, 3072, 9375, 23328, 50421, 98304, 177147, 300000, 483153, 746496, 1113879, 1613472, 2278125, 3145728, 4259571, 5668704, 7428297, 9600000, 12252303, 15460896, 19309029, 23887872
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000282";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_282(n)
    }
}

const fn power_282(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 5 {
        result *= n;
        i += 1;
    }
    3 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000282>();
}
