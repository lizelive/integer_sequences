/// a(n) = 3*T(n)^3
/// https://oeis.org/A000822

pub struct A000822;

impl crate::traits::IntegerSequence for A000822 {
    const NAME: &str = "a(n) = 3*T(n)^3";

    const HEAD: &[crate::Value] = &[
        0, 3, 81, 648, 3000, 10125, 27783, 65856, 139968, 273375, 499125, 862488, 1423656, 2260713, 3472875, 5184000, 7546368, 10744731, 15000633, 20577000, 27783000, 36979173, 48582831, 63073728, 81000000, 102984375, 129730653, 162030456, 200770248, 246938625
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000822";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_822(n)
    }
}

const fn tri_pow_822(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    3 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000822>();
}
