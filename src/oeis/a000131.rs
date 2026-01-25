/// Number of edges in n-dimensional hypercube
/// https://oeis.org/A000131

pub struct A000131;

impl crate::traits::IntegerSequence for A000131 {
    const NAME: &str = "Number of edges in n-cube";

    const HEAD: &[crate::Value] = &[
        0, 0, 1, 6, 24, 80, 240, 672, 1792, 4608, 11520, 28160, 67584, 159744, 372736, 860160, 1966080, 4456448, 10027008, 22413312, 49807360, 110100480, 242221056, 530579456, 1157627904, 2516582400, 5452595200, 11777605632, 25367150592, 54492397568
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000131";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        hypercube_edges(n)
    }
}

const fn binomial(n: isize, k: isize) -> isize {
    if k < 0 || k > n { return 0; }
    if k == 0 || k == n { return 1; }
    let k = if k > n - k { n - k } else { k };
    let mut result = 1isize;
    let mut i = 0;
    while i < k {
        result = result * (n - i) / (i + 1);
        i += 1;
    }
    result
}

const fn hypercube_edges(n: crate::Index) -> crate::Value {
    if n < 2 { return 0; }
    if n > 60 { return 0; }
    binomial(n, 2) * (1isize << (n - 2))
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000131>();
}
