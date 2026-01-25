/// a(n) = C(2n, n) = (2n)! / (n!)^2
/// https://oeis.org/A000139

pub struct A000139;

impl crate::traits::IntegerSequence for A000139 {
    const NAME: &str = "Central binomial coefficients";

    const HEAD: &[crate::Value] = &[
        1, 2, 6, 20, 70, 252, 924, 3432, 12870, 48620, 184756, 705432, 2704156, 10400600, 40116600, 155117520, 601080390, 2333606220, 9075135300, 35345263800, 137846528820, 538257874440, 2104098963720, 8233430727600, 32247603683100, 126410606437752, 495918532948104, 1946939425648112, 7648690600760440, 30067266499541040
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000139";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        central_binomial(n)
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

const fn central_binomial(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    binomial(2 * n, n)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000139>();
}
