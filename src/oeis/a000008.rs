/// Number of ways of making change for n cents using coins of 1, 2, 5, 10 cents.
/// https://oeis.org/A000008

pub struct A000008;

impl crate::traits::IntegerSequence for A000008 {
    const NAME: &str = "Number of ways of making change for n cents using coins of 1, 2, 5, 10 cents";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 2, 3, 4, 5, 6, 7, 8, 11, 12, 15, 16, 19, 22, 25, 28, 31, 34, 40, 43, 49, 52, 58,
        64, 70, 76, 82, 88, 98, 104, 114, 120, 130, 140, 150, 160, 170, 180, 195, 205, 220, 230,
        245, 260, 275, 290, 305, 320, 341, 356, 377, 392, 413, 434, 455, 476, 497, 518, 546, 567,
        595, 616, 644, 672, 700, 728, 756, 784, 820, 848, 884, 912, 948, 984, 1020, 1056, 1092,
        1128, 1173, 1209, 1254, 1290,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000008";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        change_ways(n)
    }
}

const fn change_ways(n: crate::Index) -> crate::Value {
    if n < 0 {
        return 0;
    }
    
    const MAX_N: usize = 200;
    let n = n as usize;
    if n >= MAX_N {
        return 0;
    }
    
    let coins = [1, 2, 5, 10];
    let mut dp = [0isize; MAX_N];
    dp[0] = 1;
    
    let mut c = 0;
    while c < 4 {
        let coin = coins[c] as usize;
        let mut i = coin;
        while i <= n {
            dp[i] += dp[i - coin];
            i += 1;
        }
        c += 1;
    }
    
    dp[n]
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000008>();
}
