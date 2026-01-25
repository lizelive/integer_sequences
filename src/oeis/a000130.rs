/// a(n) = n*2^n
/// https://oeis.org/A000130

pub struct A000130;

impl crate::traits::IntegerSequence for A000130 {
    const NAME: &str = "a(n) = n*2^n";

    const HEAD: &[crate::Value] = &[
        0, 2, 8, 24, 64, 160, 384, 896, 2048, 4608, 10240, 22528, 49152, 106496, 229376, 491520, 1048576, 2228224, 4718592, 9961472, 20971520, 44040192, 92274688, 192937984, 402653184, 838860800, 1744830464, 3623878656, 7516192768, 15569256448
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000130";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        n_times_2_pow_n(n)
    }
}

const fn n_times_2_pow_n(n: crate::Index) -> crate::Value {
    if n < 0 || n > 60 { return 0; }
    n * (1isize << n)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000130>();
}
