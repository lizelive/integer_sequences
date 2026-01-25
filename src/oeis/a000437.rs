/// a(n) = 8*5^n
/// https://oeis.org/A000437

pub struct A000437;

impl crate::traits::IntegerSequence for A000437 {
    const NAME: &str = "a(n) = 8*5^n";

    const HEAD: &[crate::Value] = &[
        8, 40, 200, 1000, 5000, 25000, 125000, 625000, 3125000, 15625000, 78125000, 390625000, 1953125000, 9765625000, 48828125000, 244140625000, 1220703125000, 6103515625000, 30517578125000, 152587890625000, 762939453125000, 3814697265625000, 19073486328125000, 95367431640625000, 476837158203125000
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000437";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_437(n)
    }
}

const fn pow_437(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 5;
        i += 1;
    }
    8 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000437>();
}
