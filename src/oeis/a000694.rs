/// a(n) = 1*C(n+4,10)
/// https://oeis.org/A000694

pub struct A000694;

impl crate::traits::IntegerSequence for A000694 {
    const NAME: &str = "a(n) = 1*C(n+4,10)";

    const HEAD: &[crate::Value] = &[
        0, 0, 0, 0, 0, 0, 1, 11, 66, 286, 1001, 3003, 8008, 19448, 43758, 92378, 184756, 352716, 646646, 1144066, 1961256, 3268760, 5311735, 8436285, 13123110, 20030010, 30045015, 44352165, 64512240, 92561040
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000694";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        binom_694(n)
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

const fn binom_694(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * binomial(n + 4, 10)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000694>();
}
