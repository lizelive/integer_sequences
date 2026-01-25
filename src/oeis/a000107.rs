/// Number of labeled rooted trees with n nodes (n>=1).
/// https://oeis.org/A000107

pub struct A000107;

impl crate::traits::IntegerSequence for A000107 {
    const NAME: &str = "Number of labeled rooted trees with n nodes";

    const HEAD: &[crate::Value] = &[
        0, 1, 2, 9, 64, 625, 7776, 117649, 2097152, 43046721, 1000000000, 25937424601, 743008370688, 23298085122481, 793714773254144, 29192926025390625, 1152921504606846976
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000107";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        labeled_rooted_trees(n)
    }
}

const fn labeled_rooted_trees(n: crate::Index) -> crate::Value {
    if n <= 0 { return 0; }
    if n == 1 { return 1; }
    // a(n) = n^(n-1)
    let mut result = 1isize;
    let mut i = 0;
    while i < n - 1 {
        result = result.saturating_mul(n);
        i += 1;
    }
    result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000107>();
}
