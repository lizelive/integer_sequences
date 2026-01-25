/// 5-simplex numbers: C(n+4,5)
/// https://oeis.org/A000114

pub struct A000114;

impl crate::traits::IntegerSequence for A000114 {
    const NAME: &str = "5-simplex numbers";

    const HEAD: &[crate::Value] = &[
        1, 6, 21, 56, 126, 252, 462, 792, 1287, 2002, 3003, 4368, 6188, 8568, 11628, 15504,
        20349, 26334, 33649, 42504,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000114";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        simplex_5(n)
    }
}

const fn binomial(n: isize, k: isize) -> isize {
    if k < 0 || k > n { return 0; }
    if k == 0 || k == n { return 1; }
    let k = if k > n - k { n - k } else { k };
    let mut result = 1isize;
    let mut i = 0;
    while i < k {
        result = result * (n - i) / (i + 1);
        i += 1;
    }
    result
}

const fn simplex_5(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    binomial(n + 5, 5)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000114>();
}
