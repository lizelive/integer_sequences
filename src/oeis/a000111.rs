/// Euler zigzag numbers: number of alternating permutations of {1,...,n}
/// https://oeis.org/A000111

pub struct A000111;

impl crate::traits::IntegerSequence for A000111 {
    const NAME: &str = "Euler zigzag numbers";

    const HEAD: &[crate::Value] = &[
        1, 1, 1, 2, 5, 16, 61, 272, 1385, 7936, 50521, 353792, 2702765, 22368256, 199360981,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000111";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        euler_zigzag(n)
    }
}

/// Count alternating (zigzag) permutations using the boustrophedon transform
fn euler_zigzag(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    if n == 0 { return 1; }
    if n == 1 { return 1; }
    
    const MAX: usize = 50;
    let n = n as usize;
    if n >= MAX { return 0; }
    
    // Use the boustrophedon (serpentine) algorithm
    // Row 0: starts with 1
    // Each subsequent row: if row is odd, scan left-to-right accumulating
    //                      if row is even, scan right-to-left accumulating
    
    let mut prev = [0isize; MAX];
    let mut curr = [0isize; MAX];
    prev[0] = 1;
    
    for i in 1..=n {
        if i % 2 == 1 {
            // Left to right
            curr[0] = 0;
            for j in 1..=i {
                curr[j] = curr[j-1] + prev[j-1];
            }
        } else {
            // Right to left
            curr[i] = 0;
            let mut j = i as isize - 1;
            while j >= 0 {
                let ju = j as usize;
                curr[ju] = curr[ju + 1] + prev[ju];
                j -= 1;
            }
        }
        // Copy curr to prev
        let mut j = 0;
        while j <= i {
            prev[j] = curr[j];
            j += 1;
        }
    }
    
    // Result is in the appropriate position
    if n % 2 == 1 {
        curr[n]
    } else {
        curr[0]
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000111>();
}
