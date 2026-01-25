/// Fibonacci numbers: F(n) = F(n-1) + F(n-2) with F(0) = 0 and F(1) = 1.
/// https://oeis.org/A000045

pub struct A000045;

impl crate::traits::IntegerSequence for A000045 {
    const NAME: &str = "Fibonacci numbers";

    const HEAD: &[crate::Value] = &[
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269,
        2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000045";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        fibonacci(n)
    }
}

const fn fibonacci(n: crate::Index) -> crate::Value {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    let mut a = 0isize;
    let mut b = 1isize;
    let mut i = 2;
    while i <= n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
    }
    b
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000045>();
}
