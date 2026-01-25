/// a(n) = 6*7^n
/// https://oeis.org/A000428

pub struct A000428;

impl crate::traits::IntegerSequence for A000428 {
    const NAME: &str = "a(n) = 6*7^n";

    const HEAD: &[crate::Value] = &[
        6, 42, 294, 2058, 14406, 100842, 705894, 4941258, 34588806, 242121642, 1694851494, 11863960458, 83047723206, 581334062442, 4069338437094, 28485369059658, 199397583417606, 1395783083923242, 9770481587462694, 68393371112238858, 478753597785672006
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000428";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_428(n)
    }
}

const fn pow_428(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 7;
        i += 1;
    }
    6 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000428>();
}
