/// Maximal number of regions from n points on a circle
/// https://oeis.org/A000127

pub struct A000127;

impl crate::traits::IntegerSequence for A000127 {
    const NAME: &str = "Maximal regions from n points on circle";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 4, 8, 16, 31, 57, 99, 163, 256, 386, 562, 794, 1093, 1471, 1941, 2517, 3214, 4048, 5036, 6196, 7547, 9109, 10903, 12951, 15276, 17902, 20854, 24158
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000127";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        regions_from_points(n)
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

const fn regions_from_points(n: crate::Index) -> crate::Value {
    if n <= 0 { return 1; }
    binomial(n, 4) + binomial(n, 2) + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000127>();
}
