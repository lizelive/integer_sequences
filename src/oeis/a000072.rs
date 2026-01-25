/// a(n) = n!/(n/2)!^2 = binomial(n, n/2) if n is even, 0 if n is odd.
/// https://oeis.org/A000072

pub struct A000072;

impl crate::traits::IntegerSequence for A000072 {
    const NAME: &str = "Binomial(n, floor(n/2))/2^n for n even, 0 for n odd";

    const HEAD: &[crate::Value] = &[
        1, 0, 2, 0, 6, 0, 20, 0, 70, 0, 252, 0, 924, 0, 3432, 0, 12870, 0, 48620, 0, 184756, 0,
        705432, 0, 2704156, 0, 10400600, 0, 40116600, 0, 155117520, 0, 601080390, 0, 2333606220,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000072";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if n % 2 == 1 {
            return 0;
        }
        binomial(n, n / 2)
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
    crate::tester::test_sequance_formula_matchces_head::<A000072>();
}
