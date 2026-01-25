/// a(n) = 9*T(n)^3
/// https://oeis.org/A000828

pub struct A000828;

impl crate::traits::IntegerSequence for A000828 {
    const NAME: &str = "a(n) = 9*T(n)^3";

    const HEAD: &[crate::Value] = &[
        0, 9, 243, 1944, 9000, 30375, 83349, 197568, 419904, 820125, 1497375, 2587464, 4270968, 6782139, 10418625, 15552000, 22639104, 32234193, 45001899, 61731000, 83349000, 110937519, 145748493, 189221184, 243000000, 308953125, 389191959, 486091368, 602310744, 740815875
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000828";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_828(n)
    }
}

const fn tri_pow_828(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    9 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000828>();
}
