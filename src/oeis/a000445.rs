/// a(n) = 10*2^n
/// https://oeis.org/A000445

pub struct A000445;

impl crate::traits::IntegerSequence for A000445 {
    const NAME: &str = "a(n) = 10*2^n";

    const HEAD: &[crate::Value] = &[
        10, 20, 40, 80, 160, 320, 640, 1280, 2560, 5120, 10240, 20480, 40960, 81920, 163840, 327680, 655360, 1310720, 2621440, 5242880, 10485760, 20971520, 41943040, 83886080, 167772160
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000445";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_445(n)
    }
}

const fn pow_445(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 2;
        i += 1;
    }
    10 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000445>();
}
