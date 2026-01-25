/// a(n) = 9*5^n
/// https://oeis.org/A000442

pub struct A000442;

impl crate::traits::IntegerSequence for A000442 {
    const NAME: &str = "a(n) = 9*5^n";

    const HEAD: &[crate::Value] = &[
        9, 45, 225, 1125, 5625, 28125, 140625, 703125, 3515625, 17578125, 87890625, 439453125, 2197265625, 10986328125, 54931640625, 274658203125, 1373291015625, 6866455078125, 34332275390625, 171661376953125, 858306884765625, 4291534423828125, 21457672119140625, 107288360595703125, 536441802978515625
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000442";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_442(n)
    }
}

const fn pow_442(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 5;
        i += 1;
    }
    9 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000442>();
}
