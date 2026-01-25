/// Reverse and add sequence starting from 196
/// https://oeis.org/A000124

pub struct A000124;

impl crate::traits::IntegerSequence for A000124 {
    const NAME: &str = "Reverse and add! a(1)=196, a(n) = a(n-1) + reverse(a(n-1))";

    const HEAD: &[crate::Value] = &[
        196, 887, 1675, 7436, 13783, 52514, 94039, 187088, 1067869, 10755470, 18211171
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000124";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        reverse_add_196(n)
    }
}

fn reverse_num(mut n: isize) -> isize {
    let mut rev = 0isize;
    while n > 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }
    rev
}

fn reverse_add_196(n: crate::Index) -> crate::Value {
    if n <= 0 { return 0; }
    
    let mut val = 196isize;
    for _ in 1..n {
        val = val + reverse_num(val);
        if val < 0 { return 0; } // overflow
    }
    val
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000124>();
}
