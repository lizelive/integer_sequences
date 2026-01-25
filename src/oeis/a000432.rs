/// a(n) = 7*5^n
/// https://oeis.org/A000432

pub struct A000432;

impl crate::traits::IntegerSequence for A000432 {
    const NAME: &str = "a(n) = 7*5^n";

    const HEAD: &[crate::Value] = &[
        7, 35, 175, 875, 4375, 21875, 109375, 546875, 2734375, 13671875, 68359375, 341796875, 1708984375, 8544921875, 42724609375, 213623046875, 1068115234375, 5340576171875, 26702880859375, 133514404296875, 667572021484375, 3337860107421875, 16689300537109375, 83446502685546875, 417232513427734375
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000432";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_432(n)
    }
}

const fn pow_432(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 5;
        i += 1;
    }
    7 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000432>();
}
