/// Number of partitions of n into parts each <= 5.
/// https://oeis.org/A000078

pub struct A000078;

impl crate::traits::IntegerSequence for A000078 {
    const NAME: &str = "Tetranacci numbers: a(n) = a(n-1) + a(n-2) + a(n-3) + a(n-4)";

    const HEAD: &[crate::Value] = &[
        0, 0, 0, 1, 1, 2, 4, 8, 15, 29, 56, 108, 208, 401, 773, 1490, 2872, 5536, 10671, 20569,
        39648, 76424, 147312, 283953, 547337, 1055026, 2033628, 3919944, 7555935, 14564533,
        28074040, 54114452, 104308960, 201061985, 387559437, 747044834, 1439975216, 2775641472,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000078";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        tetranacci(n)
    }
}

const fn tetranacci(n: crate::Index) -> crate::Value {
    if n < 0 {
        return 0;
    }
    if n <= 2 {
        return 0;
    }
    if n == 3 {
        return 1;
    }
    
    let mut a = 0isize;
    let mut b = 0isize;
    let mut c = 0isize;
    let mut d = 1isize;
    let mut i = 4;
    while i <= n {
        let e = a + b + c + d;
        a = b;
        b = c;
        c = d;
        d = e;
        i += 1;
    }
    d
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000078>();
}
