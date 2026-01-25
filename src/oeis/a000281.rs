/// a(n) = 2*n^5
/// https://oeis.org/A000281

pub struct A000281;

impl crate::traits::IntegerSequence for A000281 {
    const NAME: &str = "a(n) = 2*n^5";

    const HEAD: &[crate::Value] = &[
        0, 2, 64, 486, 2048, 6250, 15552, 33614, 65536, 118098, 200000, 322102, 497664, 742586, 1075648, 1518750, 2097152, 2839714, 3779136, 4952198, 6400000, 8168202, 10307264, 12872686, 15925248
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000281";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_281(n)
    }
}

const fn power_281(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 5 {
        result *= n;
        i += 1;
    }
    2 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000281>();
}
