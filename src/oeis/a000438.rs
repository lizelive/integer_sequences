/// a(n) = 8*7^n
/// https://oeis.org/A000438

pub struct A000438;

impl crate::traits::IntegerSequence for A000438 {
    const NAME: &str = "a(n) = 8*7^n";

    const HEAD: &[crate::Value] = &[
        8, 56, 392, 2744, 19208, 134456, 941192, 6588344, 46118408, 322828856, 2259801992, 15818613944, 110730297608, 775112083256, 5425784582792, 37980492079544, 265863444556808, 1861044111897656, 13027308783283592, 91191161482985144, 638338130380896008
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000438";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_438(n)
    }
}

const fn pow_438(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 7;
        i += 1;
    }
    8 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000438>();
}
