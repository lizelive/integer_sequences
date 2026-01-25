/// Number of integers from 0 to 2^n-1 with an even number of 1's in binary expansion.
/// https://oeis.org/A000047

pub struct A000047;

impl crate::traits::IntegerSequence for A000047 {
    const NAME: &str = "Number of integers from 0 to 2^n-1 with an even number of 1's in binary";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
        131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 67108864,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000047";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if n <= 0 {
            return 1;
        }
        // Number is exactly 2^(n-1)
        1 << (n - 1)
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000047>();
}
