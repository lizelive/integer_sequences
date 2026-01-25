/// a(n) = floor(n^5/5)
/// https://oeis.org/A000134

pub struct A000134;

impl crate::traits::IntegerSequence for A000134 {
    const NAME: &str = "a(n) = floor(n^5/5)";

    const HEAD: &[crate::Value] = &[
        0, 0, 6, 48, 204, 625, 1555, 3361, 6553, 11809, 20000, 32210, 49766, 74258, 107564, 151875, 209715, 283971, 377913, 495219, 640000, 816820, 1030726, 1287268, 1592524, 1953125, 2376275, 2869781, 3442073, 4102229
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000134";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        n_fifth_div_5(n)
    }
}

const fn n_fifth_div_5(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let n2 = n * n;
    n2 * n2 * n / 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000134>();
}
