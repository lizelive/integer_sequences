/// a(n) = C(n+5, 2)
/// https://oeis.org/A000336

pub struct A000336;

impl crate::traits::IntegerSequence for A000336 {
    const NAME: &str = "a(n) = C(n+5, 2)";

    const HEAD: &[crate::Value] = &[
        10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210, 231, 253, 276, 300, 325, 351, 378, 406
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000336";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        binom_336(n)
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

const fn binom_336(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    binomial(n + 5, 2)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000336>();
}
