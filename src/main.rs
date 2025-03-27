#![feature(unboxed_closures, fn_traits)]
#![allow(
    non_snake_case,
    non_camel_case_types,
    mixed_script_confusables,
    clippy::upper_case_acronyms,
    unused
)]

fn main() {
    let bf = |[a, b, c, d]: [bool; 4]| {
        let o1 = a && b || c && !d;
        let o2 = a && b && c && d;
        [o1, o2]
    };
    let vbf = VectorialBooleanFunction::new(bf);

    let mut b = [false; 4];
    for _ in 0..1 << 4 {
        let b_floats = b.map(|bi| bi as u8 as f64);
        let bf_res = bf(b).map(|xi| xi as u8 as f64);
        assert_eq!(bf_res, vbf(b_floats));

        // update input vector
        let mut carry = true;
        for bi in b.iter_mut() {
            let new_carry = carry && *bi;
            *bi ^= carry;
            carry = new_carry;
        }
    }

    println!("Passed!");
}

fn ReLU(x: f64) -> f64 {
    f64::max(0.0, x)
}

fn linear(x: f64) -> f64 {
    x
}

fn XOR(x1: f64, x2: f64) -> f64 {
    let C1 = ReLU(x2 - x1);
    let C2 = ReLU(x1 - x2);
    C1 + C2
}

#[derive(Debug)]
struct Corner<const K: usize> {
    c: f64,
    b: [bool; K],
}

impl<const K: usize> Corner<K> {
    fn compute(&self, x: [f64; K]) -> f64 {
        debug_assert!(
            self.c > 0.0 && self.c <= 1.0,
            "Invalid distance parameter: {}, should be 0 < distance <= 1",
            self.c
        );

        let sum = self
            .b
            .iter()
            .zip(x.iter())
            .map(|(&bi, &xi)| if bi { xi } else { 1.0 - xi })
            .sum::<f64>();

        ReLU(self.c.recip() * (sum - K as f64 + self.c))
    }
}

#[derive(Debug)]
struct Continuation<const K: usize> {
    corners: Vec<Corner<K>>,
}

impl<const K: usize> Continuation<K> {
    fn new(f: impl Fn([bool; K]) -> bool) -> Self {
        let c = 0.5;

        // identify the corners of f
        let mut corners: Vec<Corner<K>> = vec![];
        let mut b = [false; K];
        for _ in 0..1 << K {
            if f(b) {
                corners.push(Corner { c, b });
            }

            // update input vector
            let mut carry = true;
            for bi in b.iter_mut() {
                let new_carry = carry && *bi;
                *bi ^= carry;
                carry = new_carry;
            }
        }

        Self { corners }
    }

    fn compute(&self, x: [f64; K]) -> f64 {
        self.corners.iter().map(|corner| corner.compute(x)).sum()
    }
}

#[derive(Debug)]
struct VectorialBooleanFunction<const K: usize, const M: usize> {
    bitwise_functions: [Continuation<K>; M],
}

impl<const K: usize, const M: usize> VectorialBooleanFunction<K, M> {
    fn new(f: impl Fn([bool; K]) -> [bool; M]) -> Self {
        let bitwise_functions = (0..M)
            .map(|o| Continuation::new(|b| f(b)[o]))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Array length precondition broken");
        Self { bitwise_functions }
    }

    fn compute(&self, x: [f64; K]) -> [f64; M] {
        self.bitwise_functions
            .iter()
            .map(|f| f.compute(x))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Array length invariant broken")
    }
}

// lol, lmao
impl<const K: usize, const M: usize> FnOnce<([f64; K],)> for VectorialBooleanFunction<K, M> {
    type Output = [f64; M];

    extern "rust-call" fn call_once(self, args: ([f64; K],)) -> Self::Output {
        self.compute(args.0)
    }
}

impl<const K: usize, const M: usize> FnMut<([f64; K],)> for VectorialBooleanFunction<K, M> {
    extern "rust-call" fn call_mut(&mut self, args: ([f64; K],)) -> Self::Output {
        self.compute(args.0)
    }
}

impl<const K: usize, const M: usize> Fn<([f64; K],)> for VectorialBooleanFunction<K, M> {
    extern "rust-call" fn call(&self, args: ([f64; K],)) -> Self::Output {
        self.compute(args.0)
    }
}
