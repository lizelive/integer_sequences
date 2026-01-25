/// a(n) = 3*7^n
/// https://oeis.org/A000413

pub struct A000413;

impl crate::traits::IntegerSequence for A000413 {
    const NAME: &str = "a(n) = 3*7^n";

    const HEAD: &[crate::Value] = &[
        3, 21, 147, 1029, 7203, 50421, 352947, 2470629, 17294403, 121060821, 847425747, 5931980229, 41523861603, 290667031221, 2034669218547, 14242684529829, 99698791708803, 697891541961621, 4885240793731347, 34196685556119429, 239376798892836003
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000413";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_413(n)
    }
}

const fn pow_413(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 7;
        i += 1;
    }
    3 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000413>();
}
