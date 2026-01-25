/// a(n) = 10*T(n)^4 + 1
/// https://oeis.org/A000889

pub struct A000889;

impl crate::traits::IntegerSequence for A000889 {
    const NAME: &str = "a(n) = 10*T(n)^4 + 1";

    const HEAD: &[crate::Value] = &[
        1, 11, 811, 12961, 100001, 506251, 1944811, 6146561, 16796161, 41006251, 91506251, 189747361, 370150561, 685749611, 1215506251, 2073600001, 3421020161, 5479812811, 8550360811, 13032100001, 19448100001, 28473963211, 40971520811, 58027829761, 81000000001, 111566406251, 151784864011, 204158374561, 271709068961, 358061006251
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000889";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_889(n)
    }
}

const fn tri_pow_889(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    10 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000889>();
}
