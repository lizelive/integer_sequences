/// a(n) = 2^(n-1).
/// https://oeis.org/A000079

pub struct A000079;

impl crate::traits::IntegerSequence for A000079 {
    const NAME: &str = "Powers of 2: a(n) = 2^n";

    const HEAD: &[crate::Value] = &[
        1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072,
        262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728,
        268435456, 536870912, 1073741824, 2147483648, 4294967296, 8589934592, 17179869184,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000079";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        powers_of_2(n)
    }
}

const fn powers_of_2(n: crate::Index) -> crate::Value {
    if n < 0 {
        return 0;
    }
    1 << n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000079>();
}
