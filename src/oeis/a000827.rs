/// a(n) = 8*T(n)^3
/// https://oeis.org/A000827

pub struct A000827;

impl crate::traits::IntegerSequence for A000827 {
    const NAME: &str = "a(n) = 8*T(n)^3";

    const HEAD: &[crate::Value] = &[
        0, 8, 216, 1728, 8000, 27000, 74088, 175616, 373248, 729000, 1331000, 2299968, 3796416, 6028568, 9261000, 13824000, 20123648, 28652616, 40001688, 54872000, 74088000, 98611128, 129554216, 168196608, 216000000, 274625000, 345948408, 432081216, 535387328, 658503000
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000827";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_827(n)
    }
}

const fn tri_pow_827(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    8 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000827>();
}
