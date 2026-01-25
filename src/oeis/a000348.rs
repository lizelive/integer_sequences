/// a(n) = C(n+6, 7)
/// https://oeis.org/A000348

pub struct A000348;

impl crate::traits::IntegerSequence for A000348 {
    const NAME: &str = "a(n) = C(n+6, 7)";

    const HEAD: &[crate::Value] = &[
        0, 1, 8, 36, 120, 330, 792, 1716, 3432, 6435, 11440, 19448, 31824, 50388, 77520, 116280, 170544, 245157, 346104, 480700, 657800, 888030, 1184040, 1560780, 2035800
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000348";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        binom_348(n)
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

const fn binom_348(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    binomial(n + 6, 7)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000348>();
}
