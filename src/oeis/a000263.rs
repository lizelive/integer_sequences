/// a(n) = 4*n^3
/// https://oeis.org/A000263

pub struct A000263;

impl crate::traits::IntegerSequence for A000263 {
    const NAME: &str = "a(n) = 4*n^3";

    const HEAD: &[crate::Value] = &[
        0, 4, 32, 108, 256, 500, 864, 1372, 2048, 2916, 4000, 5324, 6912, 8788, 10976, 13500, 16384, 19652, 23328, 27436, 32000, 37044, 42592, 48668, 55296
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000263";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_263(n)
    }
}

const fn power_263(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 3 {
        result *= n;
        i += 1;
    }
    4 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000263>();
}
