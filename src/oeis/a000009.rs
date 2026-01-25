/// Number of partitions of n into distinct parts; number of partitions of n into odd parts.
/// https://oeis.org/A000009

pub struct A000009;

impl crate::traits::IntegerSequence for A000009 {
    const NAME: &str = "Number of partitions of n into distinct parts";

    const HEAD: &[crate::Value] = &[
        1, 1, 1, 2, 2, 3, 4, 5, 6, 8, 10, 12, 15, 18, 22, 27, 32, 38, 46, 54, 64, 76, 89, 104, 122,
        142, 165, 192, 222, 256, 296, 340, 390, 448, 512, 585, 668, 760, 864, 982, 1113, 1260,
        1426, 1610, 1816, 2048, 2304, 2590, 2910, 3264, 3658, 4097, 4582, 5120, 5718, 6378, 7108,
        7917, 8808, 9792, 10880, 12076, 13394, 14848, 16444, 18200, 20132, 22250, 24576, 27130,
        29927, 32992, 36352, 40026, 44046, 48446, 53250, 58499, 64234, 70488, 77312, 84756, 92864,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000009";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        partitions_distinct(n)
    }
}

const fn partitions_distinct(n: crate::Index) -> crate::Value {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 1;
    }
    
    const MAX_N: usize = 200;
    let n = n as usize;
    if n >= MAX_N {
        return 0;
    }
    
    let mut dp = [0isize; MAX_N];
    dp[0] = 1;
    
    let mut part = 1usize;
    while part <= n {
        let mut i = n;
        while i >= part {
            dp[i] += dp[i - part];
            i -= 1;
        }
        part += 1;
    }
    
    dp[n]
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000009>();
}
