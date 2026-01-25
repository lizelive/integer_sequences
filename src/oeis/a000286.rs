/// a(n) = 7*n^5
/// https://oeis.org/A000286

pub struct A000286;

impl crate::traits::IntegerSequence for A000286 {
    const NAME: &str = "a(n) = 7*n^5";

    const HEAD: &[crate::Value] = &[
        0, 7, 224, 1701, 7168, 21875, 54432, 117649, 229376, 413343, 700000, 1127357, 1741824, 2599051, 3764768, 5315625, 7340032, 9938999, 13226976, 17332693, 22400000, 28588707, 36075424, 45054401, 55738368
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000286";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_286(n)
    }
}

const fn power_286(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 5 {
        result *= n;
        i += 1;
    }
    7 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000286>();
}
