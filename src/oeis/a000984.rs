/// Central binomial coefficients: binomial(2*n,n) = (2*n)!/(n!)^2.
/// https://oeis.org/A000984

pub struct A000984;

impl crate::traits::IntegerSequence for A000984 {
    const NAME: &str = "Central binomial coefficients: binomial(2*n,n)";

    const HEAD: &[crate::Value] = &[
        1, 2, 6, 20, 70, 252, 924, 3432, 12870, 48620, 184756, 705432, 2704156, 10400600, 40116600,
        155117520, 601080390, 2333606220, 9075135300, 35345263800, 137846528820, 538257874440,
        2104098963720, 8233430727600, 32247603683100, 126410606437752, 495918532948104,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000984";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        binomial(2 * n, n)
    }
}

const fn binomial(n: crate::Index, k: crate::Index) -> crate::Value {
    if k < 0 || k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    
    let k = if k > n - k { n - k } else { k };
    let mut result = 1isize;
    let mut i = 0;
    while i < k {
        result = result * (n - i) / (i + 1);
        i += 1;
    }
    result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000984>();
}
