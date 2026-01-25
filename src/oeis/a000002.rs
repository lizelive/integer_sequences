/// Kolakoski sequence: a(n) is length of n-th run; a(1) = 1; sequence consists of 1's and 2's only.
/// https://oeis.org/A000002

pub struct A000002;

impl crate::traits::IntegerSequence for A000002 {
    const NAME: &str = "Kolakoski sequence";

    const HEAD: &[crate::Value] = &[
        1, 2, 2, 1, 1, 2, 1, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 2, 2, 1, 2, 1, 1, 2, 1, 2, 2, 1, 1, 2,
        1, 1, 2, 1, 2, 2, 1, 2, 2, 1, 1, 2, 1, 2, 2, 1, 2, 1, 1, 2, 1, 1, 2, 2, 1, 2, 2, 1, 1, 2,
        1, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 2, 1, 2, 2, 1, 2, 1, 1, 2, 2, 1, 2, 2, 1, 1, 2, 1, 2, 2,
        1, 2, 2, 1, 1, 2, 1, 1, 2, 2,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000002";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        kolakoski(n)
    }
}

const fn kolakoski(n: crate::Index) -> crate::Value {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    if n == 2 || n == 3 {
        return 2;
    }

    // Generate the sequence iteratively up to position n
    const MAX_SIZE: usize = 500;
    let mut seq = [0isize; MAX_SIZE];
    seq[0] = 1;
    seq[1] = 2;
    seq[2] = 2;

    let mut length = 3usize;
    let mut run_index = 2usize;
    let mut current_symbol = 1isize;

    let n_idx = n as usize;
    if n_idx > MAX_SIZE {
        return 0;
    }

    while length < n_idx {
        let run_length = seq[run_index] as usize;
        let mut i = 0;
        while i < run_length && length < n_idx {
            seq[length] = current_symbol;
            length += 1;
            i += 1;
        }
        current_symbol = 3 - current_symbol;
        run_index += 1;
    }

    seq[n_idx - 1]
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000002>();
}
