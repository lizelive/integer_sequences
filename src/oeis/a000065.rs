/// Number of nonisomorphic semigroups with n elements.
/// https://oeis.org/A000065

pub struct A000065;

impl crate::traits::IntegerSequence for A000065 {
    const NAME: &str = "a(n) = -1 + number of partitions of n";

    const HEAD: &[crate::Value] = &[
        0, 0, 1, 2, 4, 6, 10, 14, 21, 29, 41, 55, 76, 100, 134, 175, 230, 296, 384, 489, 626, 791,
        1001, 1254, 1574, 1957, 2435, 3009, 3717, 4564, 5603, 6841, 8348, 10142, 12309, 14882,
        17976, 21636, 26014, 31184, 37337, 44582, 53173, 63260, 75174, 89133, 105557, 124753,
        147272, 173524,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000065";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        partition(n) - 1
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
    crate::tester::test_sequance_formula_matchces_head::<A000065>();
}
