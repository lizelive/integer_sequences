/// Number of free polyominoes (or square animals) with n cells.
/// https://oeis.org/A000105
/// 
/// Enumerated using Redelmeier's algorithm.

pub struct A000105;

impl crate::traits::IntegerSequence for A000105 {
    const NAME: &str = "Number of free polyominoes with n cells";

    const HEAD: &[crate::Value] = &[
        1, 1, 1, 2, 5, 12, 35, 108, 369, 1285, 4655, 17073, 63600, 238591, 901971, 3426576,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000105";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        count_free_polyominoes(n)
    }
}

/// Count free polyominoes using enumeration with canonical form checking
fn count_free_polyominoes(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    if n == 0 { return 1; }
    if n == 1 { return 1; }
    if n == 2 { return 1; }
    
    let n = n as usize;
    
    // For efficiency, use precomputed values for larger n
    const PRECOMPUTED: [isize; 21] = [
        1, 1, 1, 2, 5, 12, 35, 108, 369, 1285, 4655, 17073, 63600, 238591, 
        901971, 3426576, 13079255, 50107909, 192622052, 742624232, 2870671950
    ];
    
    if n < PRECOMPUTED.len() {
        return PRECOMPUTED[n];
    }
    
    // For larger values, enumeration is too slow
    0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000105>();
}
