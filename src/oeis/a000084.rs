/// Number of planar partitions of n.
/// https://oeis.org/A000084

pub struct A000084;

impl crate::traits::IntegerSequence for A000084 {
    const NAME: &str = "Number of series-parallel networks with n unlabeled edges";

    const HEAD: &[crate::Value] = &[
        1, 2, 4, 10, 24, 66, 180, 522, 1532, 4624, 14136, 43930, 137908, 437502, 1399068, 4507352,
        14611576, 47633486, 156047204, 513477502, 1696305728, 5623993944, 18706733128, 62408176762,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000084";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        0
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000084>();
}
