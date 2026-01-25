/// a(n) = 9*2^n
/// https://oeis.org/A000440

pub struct A000440;

impl crate::traits::IntegerSequence for A000440 {
    const NAME: &str = "a(n) = 9*2^n";

    const HEAD: &[crate::Value] = &[
        9, 18, 36, 72, 144, 288, 576, 1152, 2304, 4608, 9216, 18432, 36864, 73728, 147456, 294912, 589824, 1179648, 2359296, 4718592, 9437184, 18874368, 37748736, 75497472, 150994944
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000440";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_440(n)
    }
}

const fn pow_440(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 2;
        i += 1;
    }
    9 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000440>();
}
