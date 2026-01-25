/// Primorials
/// https://oeis.org/A000122

pub struct A000122;

impl crate::traits::IntegerSequence for A000122 {
    const NAME: &str = "The canonical primorial (product of first n primes)";

    const HEAD: &[crate::Value] = &[
        1, 2, 6, 30, 210, 2310, 30030, 510510, 9699690, 223092870
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000122";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        primorial(n)
    }
}

const fn is_prime(n: isize) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}

fn primorial(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    if n == 0 { return 1; }
    
    let mut result = 1isize;
    let mut count = 0isize;
    let mut p = 2isize;
    
    while count < n && result > 0 {
        if is_prime(p) {
            result = result.saturating_mul(p);
            count += 1;
        }
        p += 1;
    }
    result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000122>();
}
