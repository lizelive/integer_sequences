/// Partial sums of (unordered) prime signature of n.
/// https://oeis.org/A000064

pub struct A000064;

impl crate::traits::IntegerSequence for A000064 {
    const NAME: &str = "Partial sums of A000005 (divisor function)";

    const HEAD: &[crate::Value] = &[
        1, 3, 5, 8, 10, 14, 16, 20, 23, 27, 29, 35, 37, 41, 45, 50, 52, 58, 60, 66, 70, 74, 76, 84,
        87, 91, 95, 101, 103, 111, 113, 119, 123, 127, 131, 140, 142, 146, 150, 158, 160, 168, 170,
        176, 182, 186, 188, 198, 201, 207, 211, 217, 219, 227, 231, 239, 243, 247, 249, 261, 263,
        267, 273, 280, 284, 292, 294, 300, 304, 312, 314, 326, 328, 332, 338, 344, 348, 356, 358,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000064";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        sum_divisor_counts(n)
    }
}

const fn divisor_count(n: crate::Index) -> crate::Value {
    if n <= 0 {
        return 0;
    }
    let mut count = 0;
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            count += 1;
            if i != n / i {
                count += 1;
            }
        }
        i += 1;
    }
    count
}

const fn sum_divisor_counts(n: crate::Index) -> crate::Value {
    let mut sum = 0;
    let mut i = 1;
    while i <= n {
        sum += divisor_count(i);
        i += 1;
    }
    sum
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000064>();
}
