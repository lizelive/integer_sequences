/// a(n) = 1*C(n+2,9)
/// https://oeis.org/A000682

pub struct A000682;

impl crate::traits::IntegerSequence for A000682 {
    const NAME: &str = "a(n) = 1*C(n+2,9)";

    const HEAD: &[crate::Value] = &[
        0, 0, 0, 0, 0, 0, 0, 1, 10, 55, 220, 715, 2002, 5005, 11440, 24310, 48620, 92378, 167960, 293930, 497420, 817190, 1307504, 2042975, 3124550, 4686825, 6906900, 10015005, 14307150, 20160075
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000682";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        binom_682(n)
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

const fn binom_682(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * binomial(n + 2, 9)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000682>();
}
