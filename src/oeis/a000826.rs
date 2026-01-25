/// a(n) = 7*T(n)^3
/// https://oeis.org/A000826

pub struct A000826;

impl crate::traits::IntegerSequence for A000826 {
    const NAME: &str = "a(n) = 7*T(n)^3";

    const HEAD: &[crate::Value] = &[
        0, 7, 189, 1512, 7000, 23625, 64827, 153664, 326592, 637875, 1164625, 2012472, 3321864, 5274997, 8103375, 12096000, 17608192, 25071039, 35001477, 48013000, 64827000, 86284737, 113359939, 147172032, 189000000, 240296875, 302704857, 378071064, 468463912, 576190125
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000826";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_826(n)
    }
}

const fn tri_pow_826(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    7 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000826>();
}
