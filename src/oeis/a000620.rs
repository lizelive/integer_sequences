/// a(n) = 1*C(n+0,3)
/// https://oeis.org/A000620

pub struct A000620;

impl crate::traits::IntegerSequence for A000620 {
    const NAME: &str = "a(n) = 1*C(n+0,3)";

    const HEAD: &[crate::Value] = &[
        0, 0, 0, 1, 4, 10, 20, 35, 56, 84, 120, 165, 220, 286, 364, 455, 560, 680, 816, 969, 1140, 1330, 1540, 1771, 2024, 2300, 2600, 2925, 3276, 3654
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000620";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        binom_620(n)
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

const fn binom_620(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * binomial(n + 0, 3)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000620>();
}
