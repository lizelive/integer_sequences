/// Number of n-element quasigroups.
/// https://oeis.org/A000069

pub struct A000069;

impl crate::traits::IntegerSequence for A000069 {
    const NAME: &str = "Odious numbers: positive integers with an odd number of 1's in binary";

    const HEAD: &[crate::Value] = &[
        1, 2, 4, 7, 8, 11, 13, 14, 16, 19, 21, 22, 25, 26, 28, 31, 32, 35, 37, 38, 41, 42, 44, 47,
        49, 50, 52, 55, 56, 59, 61, 62, 64, 67, 69, 70, 73, 74, 76, 79, 81, 82, 84, 87, 88, 91, 93,
        94, 97, 98, 100, 103, 104, 107, 109, 110, 112, 115, 117, 118, 121, 122, 124, 127, 128,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000069";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        nth_odious(n)
    }
}

const fn popcount(mut n: crate::Index) -> crate::Value {
    let mut count = 0;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

const fn is_odious(n: crate::Index) -> bool {
    popcount(n) % 2 == 1
}

const fn nth_odious(n: crate::Index) -> crate::Value {
    if n <= 0 {
        return 0;
    }
    let mut count = 0;
    let mut candidate = 1;
    while count < n {
        if is_odious(candidate) {
            count += 1;
            if count == n {
                return candidate;
            }
        }
        candidate += 1;
    }
    0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000069>();
}
