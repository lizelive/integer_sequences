/// a(n) = 7*T(n)^5
/// https://oeis.org/A000846

pub struct A000846;

impl crate::traits::IntegerSequence for A000846 {
    const NAME: &str = "a(n) = 7*T(n)^5";

    const HEAD: &[crate::Value] = &[
        0, 7, 1701, 54432, 700000, 5315625, 28588707, 120472576, 423263232, 1291696875, 3522990625, 8766328032, 20210220576, 43682250157, 89339709375, 174182400000, 325681119232, 586887951951, 1023478188957, 1733269300000, 2858870700000, 4604239851057, 7256056335451, 11210976709632, 17010000000000, 25381357421875, 37293541087257, 54020305908576, 77219717398432, 109029576403125
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000846";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_846(n)
    }
}

const fn tri_pow_846(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    7 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000846>();
}
