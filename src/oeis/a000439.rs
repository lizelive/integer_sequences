/// a(n) = 8*11^n
/// https://oeis.org/A000439

pub struct A000439;

impl crate::traits::IntegerSequence for A000439 {
    const NAME: &str = "a(n) = 8*11^n";

    const HEAD: &[crate::Value] = &[
        8, 88, 968, 10648, 117128, 1288408, 14172488, 155897368, 1714871048, 18863581528, 207499396808, 2282493364888, 25107427013768, 276181697151448, 3037998668665928, 33417985355325208, 367597838908577288
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000439";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_439(n)
    }
}

const fn pow_439(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 11;
        i += 1;
    }
    8 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000439>();
}
