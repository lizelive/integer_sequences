/// Central binomial coefficients
/// https://oeis.org/A000125

pub struct A000125;

impl crate::traits::IntegerSequence for A000125 {
    const NAME: &str = "Central column of Pascal's triangle: a(n) = C(2n,n)";

    const HEAD: &[crate::Value] = &[
        1, 2, 6, 20, 70, 252, 924, 3432, 12870, 48620, 184756, 705432, 2704156, 10400600
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000125";

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
    crate::tester::test_sequance_formula_matchces_head::<A000125>();
}
