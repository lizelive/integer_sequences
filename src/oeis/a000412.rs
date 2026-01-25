/// a(n) = 3*5^n
/// https://oeis.org/A000412

pub struct A000412;

impl crate::traits::IntegerSequence for A000412 {
    const NAME: &str = "a(n) = 3*5^n";

    const HEAD: &[crate::Value] = &[
        3, 15, 75, 375, 1875, 9375, 46875, 234375, 1171875, 5859375, 29296875, 146484375, 732421875, 3662109375, 18310546875, 91552734375, 457763671875, 2288818359375, 11444091796875, 57220458984375, 286102294921875, 1430511474609375, 7152557373046875, 35762786865234375, 178813934326171875
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000412";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_412(n)
    }
}

const fn pow_412(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 5;
        i += 1;
    }
    3 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000412>();
}
