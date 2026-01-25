/// a(n) = 3*2^n
/// https://oeis.org/A000410

pub struct A000410;

impl crate::traits::IntegerSequence for A000410 {
    const NAME: &str = "a(n) = 3*2^n";

    const HEAD: &[crate::Value] = &[
        3, 6, 12, 24, 48, 96, 192, 384, 768, 1536, 3072, 6144, 12288, 24576, 49152, 98304, 196608, 393216, 786432, 1572864, 3145728, 6291456, 12582912, 25165824, 50331648
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000410";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_410(n)
    }
}

const fn pow_410(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 2;
        i += 1;
    }
    3 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000410>();
}
