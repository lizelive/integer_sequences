/// a(n) = C(n+4, 6)
/// https://oeis.org/A000333

pub struct A000333;

impl crate::traits::IntegerSequence for A000333 {
    const NAME: &str = "a(n) = C(n+4, 6)";

    const HEAD: &[crate::Value] = &[
        0, 0, 1, 7, 28, 84, 210, 462, 924, 1716, 3003, 5005, 8008, 12376, 18564, 27132, 38760, 54264, 74613, 100947, 134596, 177100, 230230, 296010, 376740
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000333";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        binom_333(n)
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

const fn binom_333(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    binomial(n + 4, 6)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000333>();
}
