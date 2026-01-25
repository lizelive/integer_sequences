/// a(n) = 3*n^6
/// https://oeis.org/A000292

pub struct A000292;

impl crate::traits::IntegerSequence for A000292 {
    const NAME: &str = "a(n) = 3*n^6";

    const HEAD: &[crate::Value] = &[
        0, 3, 192, 2187, 12288, 46875, 139968, 352947, 786432, 1594323, 3000000, 5314683, 8957952, 14480427, 22588608, 34171875, 50331648, 72412707, 102036672, 141137643, 192000000, 257298363, 340139712, 444107667, 573308928
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000292";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_292(n)
    }
}

const fn power_292(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 6 {
        result *= n;
        i += 1;
    }
    3 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000292>();
}
