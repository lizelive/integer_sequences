/// a(n) = 7*T(n)^5 + 1
/// https://oeis.org/A000896

pub struct A000896;

impl crate::traits::IntegerSequence for A000896 {
    const NAME: &str = "a(n) = 7*T(n)^5 + 1";

    const HEAD: &[crate::Value] = &[
        1, 8, 1702, 54433, 700001, 5315626, 28588708, 120472577, 423263233, 1291696876, 3522990626, 8766328033, 20210220577, 43682250158, 89339709376, 174182400001, 325681119233, 586887951952, 1023478188958, 1733269300001, 2858870700001, 4604239851058, 7256056335452, 11210976709633, 17010000000001, 25381357421876, 37293541087258, 54020305908577, 77219717398433, 109029576403126
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000896";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_896(n)
    }
}

const fn tri_pow_896(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    7 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000896>();
}
