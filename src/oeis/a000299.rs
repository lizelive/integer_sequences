/// a(n) = 10*n^6
/// https://oeis.org/A000299

pub struct A000299;

impl crate::traits::IntegerSequence for A000299 {
    const NAME: &str = "a(n) = 10*n^6";

    const HEAD: &[crate::Value] = &[
        0, 10, 640, 7290, 40960, 156250, 466560, 1176490, 2621440, 5314410, 10000000, 17715610, 29859840, 48268090, 75295360, 113906250, 167772160, 241375690, 340122240, 470458810, 640000000, 857661210, 1133799040, 1480358890, 1911029760
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000299";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_299(n)
    }
}

const fn power_299(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 6 {
        result *= n;
        i += 1;
    }
    10 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000299>();
}
