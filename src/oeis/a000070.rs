/// a(n) = 2^n - n - 1.
/// https://oeis.org/A000070

pub struct A000070;

impl crate::traits::IntegerSequence for A000070 {
    const NAME: &str = "a(n) = Sum_{k=0..n} p(k) where p(k) = number of partitions of k";

    const HEAD: &[crate::Value] = &[
        1, 2, 4, 7, 12, 19, 30, 45, 67, 97, 139, 195, 272, 373, 508, 684, 915, 1212, 1597, 2087,
        2714, 3506, 4508, 5763, 7338, 9296, 11732, 14742, 18460, 23025, 28629, 35471, 43820, 53963,
        66273, 81156, 99133, 120770, 146785, 177970, 215308, 259891, 313065, 376326, 451501,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000070";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        sum_partitions(n)
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

const fn sum_partitions(n: crate::Index) -> crate::Value {
    let mut sum = 0;
    let mut k = 0;
    while k <= n {
        sum += partition(k);
        k += 1;
    }
    sum
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000070>();
}
