/// a(n) = 1*C(n+9,9)
/// https://oeis.org/A000689

pub struct A000689;

impl crate::traits::IntegerSequence for A000689 {
    const NAME: &str = "a(n) = 1*C(n+9,9)";

    const HEAD: &[crate::Value] = &[
        1, 10, 55, 220, 715, 2002, 5005, 11440, 24310, 48620, 92378, 167960, 293930, 497420, 817190, 1307504, 2042975, 3124550, 4686825, 6906900, 10015005, 14307150, 20160075, 28048800, 38567100, 52451256, 70607460, 94143280, 124403620, 163011640
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000689";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        binom_689(n)
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

const fn binom_689(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * binomial(n + 9, 9)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000689>();
}
