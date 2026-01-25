/// a(n) = 1*C(n+0,8)
/// https://oeis.org/A000670

pub struct A000670;

impl crate::traits::IntegerSequence for A000670 {
    const NAME: &str = "a(n) = 1*C(n+0,8)";

    const HEAD: &[crate::Value] = &[
        0, 0, 0, 0, 0, 0, 0, 0, 1, 9, 45, 165, 495, 1287, 3003, 6435, 12870, 24310, 43758, 75582, 125970, 203490, 319770, 490314, 735471, 1081575, 1562275, 2220075, 3108105, 4292145
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000670";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        binom_670(n)
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

const fn binom_670(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * binomial(n + 0, 8)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000670>();
}
