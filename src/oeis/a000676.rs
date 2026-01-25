/// a(n) = 1*C(n+6,8)
/// https://oeis.org/A000676

pub struct A000676;

impl crate::traits::IntegerSequence for A000676 {
    const NAME: &str = "a(n) = 1*C(n+6,8)";

    const HEAD: &[crate::Value] = &[
        0, 0, 1, 9, 45, 165, 495, 1287, 3003, 6435, 12870, 24310, 43758, 75582, 125970, 203490, 319770, 490314, 735471, 1081575, 1562275, 2220075, 3108105, 4292145, 5852925, 7888725, 10518300, 13884156, 18156204, 23535820
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000676";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        binom_676(n)
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

const fn binom_676(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * binomial(n + 6, 8)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000676>();
}
