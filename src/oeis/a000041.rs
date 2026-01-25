/// a(n) is the number of partitions of n (the partition numbers).
/// https://oeis.org/A000041

pub struct A000041;

impl crate::traits::IntegerSequence for A000041 {
    const NAME: &str = "a(n) is the number of partitions of n";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627, 792,
        1002, 1255, 1575, 1958, 2436, 3010, 3718, 4565, 5604, 6842, 8349, 10143, 12310, 14883,
        17977, 21637, 26015, 31185, 37338, 44583, 53174, 63261, 75175, 89134, 105558, 124754,
        147273, 173525,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000041";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        partition(n)
    }
}

const fn partition(n: crate::Index) -> crate::Value {
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
        let mut i = part;
        while i <= n {
            dp[i] += dp[i - part];
            i += 1;
        }
        part += 1;
    }
    
    dp[n]
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000041>();
}
