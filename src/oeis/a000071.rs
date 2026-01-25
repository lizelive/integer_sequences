/// Number of permutations of {1,2,...,n} with exactly 2 fixed points.
/// https://oeis.org/A000071

pub struct A000071;

impl crate::traits::IntegerSequence for A000071 {
    const NAME: &str = "a(n) = Fibonacci(n) - 1";

    const HEAD: &[crate::Value] = &[
        0, 0, 0, 1, 2, 4, 7, 12, 20, 33, 54, 88, 143, 232, 376, 609, 986, 1596, 2583, 4180, 6764,
        10945, 17710, 28656, 46367, 75024, 121392, 196417, 317810, 514228, 832039, 1346268,
        2178308, 3524577, 5702886, 9227464, 14930351, 24157816, 39088168, 63245985, 102334154,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000071";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        // Use HEAD for known values
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        if n <= 1 {
            return 0;
        }
        fibonacci(n) - 1
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
    crate::tester::test_sequance_formula_matchces_head::<A000071>();
}
