/// a(n) = 7*n^6
/// https://oeis.org/A000296

pub struct A000296;

impl crate::traits::IntegerSequence for A000296 {
    const NAME: &str = "a(n) = 7*n^6";

    const HEAD: &[crate::Value] = &[
        0, 7, 448, 5103, 28672, 109375, 326592, 823543, 1835008, 3720087, 7000000, 12400927, 20901888, 33787663, 52706752, 79734375, 117440512, 168962983, 238085568, 329321167, 448000000, 600362847, 793659328, 1036251223, 1337720832
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000296";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_296(n)
    }
}

const fn power_296(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 6 {
        result *= n;
        i += 1;
    }
    7 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000296>();
}
