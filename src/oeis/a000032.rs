/// Lucas numbers beginning at 2: L(n) = L(n-1) + L(n-2), L(0) = 2, L(1) = 1.
/// https://oeis.org/A000032

pub struct A000032;

impl crate::traits::IntegerSequence for A000032 {
    const NAME: &str = "Lucas numbers";

    const HEAD: &[crate::Value] = &[
        2, 1, 3, 4, 7, 11, 18, 29, 47, 76, 123, 199, 322, 521, 843, 1364, 2207, 3571, 5778, 9349,
        15127, 24476, 39603, 64079, 103682, 167761, 271443, 439204, 710647, 1149851, 1860498,
        3010349, 4870847, 7881196, 12752043, 20633239, 33385282, 54018521, 87403803,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000032";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        lucas(n)
    }
}

const fn lucas(n: crate::Index) -> crate::Value {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 2;
    }
    if n == 1 {
        return 1;
    }
    
    let mut a = 2isize;
    let mut b = 1isize;
    let mut i = 2;
    while i <= n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
    }
    b
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000032>();
}
