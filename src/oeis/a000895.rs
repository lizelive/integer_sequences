/// a(n) = 6*T(n)^5 + 1
/// https://oeis.org/A000895

pub struct A000895;

impl crate::traits::IntegerSequence for A000895 {
    const NAME: &str = "a(n) = 6*T(n)^5 + 1";

    const HEAD: &[crate::Value] = &[
        1, 7, 1459, 46657, 600001, 4556251, 24504607, 103262209, 362797057, 1107168751, 3019706251, 7513995457, 17323046209, 37441928707, 76576893751, 149299200001, 279155245057, 503046815959, 877267019107, 1485659400001, 2450460600001, 3946491300907, 6219476858959, 9609408608257, 14580000000001, 21755449218751, 31965892360507, 46303119350209, 66188329198657, 93453922631251
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000895";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_895(n)
    }
}

const fn tri_pow_895(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    6 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000895>();
}
