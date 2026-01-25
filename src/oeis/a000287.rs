/// a(n) = 8*n^5
/// https://oeis.org/A000287

pub struct A000287;

impl crate::traits::IntegerSequence for A000287 {
    const NAME: &str = "a(n) = 8*n^5";

    const HEAD: &[crate::Value] = &[
        0, 8, 256, 1944, 8192, 25000, 62208, 134456, 262144, 472392, 800000, 1288408, 1990656, 2970344, 4302592, 6075000, 8388608, 11358856, 15116544, 19808792, 25600000, 32672808, 41229056, 51490744, 63700992
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000287";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_287(n)
    }
}

const fn power_287(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 5 {
        result *= n;
        i += 1;
    }
    8 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000287>();
}
