/// Number of non-crossing partitions of n
/// https://oeis.org/A000113

pub struct A000113;

impl crate::traits::IntegerSequence for A000113 {
    const NAME: &str = "Number of non-crossing partitions of n";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796, 58786, 208012, 742900, 2674440
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000113";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        catalan(n)
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

const fn catalan(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    binomial(2 * n, n) / (n + 1)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000113>();
}
