/// Denumerants
/// https://oeis.org/A000115

pub struct A000115;

impl crate::traits::IntegerSequence for A000115 {
    const NAME: &str = "Denumerants: a(n) = # ways to pay n cents using 1,2,5,10 cent coins";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 2, 3, 4, 5, 6, 7, 8, 11, 12, 15, 16, 19, 22, 25, 28, 31, 34, 40, 43, 49, 52, 58
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000115";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        denumerants_1_2_5_10(n)
    }
}

fn denumerants_1_2_5_10(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let n = n as usize;
    
    const MAX: usize = 1000;
    if n >= MAX { return 0; }
    
    let mut dp = [0isize; MAX];
    dp[0] = 1;
    
    let coins = [1usize, 2, 5, 10];
    for &coin in &coins {
        for i in coin..=n {
            dp[i] += dp[i - coin];
        }
    }
    dp[n]
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000115>();
}
