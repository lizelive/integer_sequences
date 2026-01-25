/// a(n) = 4*T(n)^3 + 1
/// https://oeis.org/A000873

pub struct A000873;

impl crate::traits::IntegerSequence for A000873 {
    const NAME: &str = "a(n) = 4*T(n)^3 + 1";

    const HEAD: &[crate::Value] = &[
        1, 5, 109, 865, 4001, 13501, 37045, 87809, 186625, 364501, 665501, 1149985, 1898209, 3014285, 4630501, 6912001, 10061825, 14326309, 20000845, 27436001, 37044001, 49305565, 64777109, 84098305, 108000001, 137312501, 172974205, 216040609, 267693665, 329251501
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000873";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_873(n)
    }
}

const fn tri_pow_873(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    4 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000873>();
}
