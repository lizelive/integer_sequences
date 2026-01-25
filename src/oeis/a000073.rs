/// Number of even permutations of length n with no fixed points.
/// https://oeis.org/A000073

pub struct A000073;

impl crate::traits::IntegerSequence for A000073 {
    const NAME: &str = "Tribonacci numbers: a(n) = a(n-1) + a(n-2) + a(n-3)";

    const HEAD: &[crate::Value] = &[
        0, 0, 1, 1, 2, 4, 7, 13, 24, 44, 81, 149, 274, 504, 927, 1705, 3136, 5768, 10609, 19513,
        35890, 66012, 121415, 223317, 410744, 755476, 1389537, 2555757, 4700770, 8646064, 15902591,
        29249425, 53798080, 98950096, 181997601, 334745777, 615693474, 1132436852,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000073";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        tribonacci(n)
    }
}

const fn tribonacci(n: crate::Index) -> crate::Value {
    if n < 0 {
        return 0;
    }
    if n == 0 || n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    
    let mut a = 0isize;
    let mut b = 0isize;
    let mut c = 1isize;
    let mut i = 3;
    while i <= n {
        let d = a + b + c;
        a = b;
        b = c;
        c = d;
        i += 1;
    }
    c
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000073>();
}
