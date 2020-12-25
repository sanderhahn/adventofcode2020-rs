// https://en.wikipedia.org/wiki/Heap%27s_algorithm
struct Permutation {
    next: usize,
    i: usize,
    k: usize,
    stack: Vec<usize>,
}

impl Permutation {
    fn new(vec: &Vec<usize>) -> Self {
        let k = vec.len();
        Self {
            next: 0,
            i: 0,
            k,
            stack: vec![0; k],
        }
    }
}

impl Permutation {
    fn next_with_vec(&mut self, vec: &mut Vec<usize>) -> bool {
        if self.next == 0 {
            self.next += 1;
            return true;
        }
        while self.i < self.k {
            if self.stack[self.i] < self.i {
                if (self.i & 1) == 0 {
                    vec.swap(0, self.i);
                } else {
                    vec.swap(self.stack[self.i], self.i);
                }
                self.stack[self.i] += 1;
                self.i = 0;
                self.next += 1;
                return true;
            } else {
                self.stack[self.i] = 0;
                self.i += 1;
            }
        }
        false
    }
}

fn test_permutation() {
    let mut items = vec![1, 2, 3];
    let mut permutate = Permutation::new(&items);
    let mut result = vec![];
    while permutate.next_with_vec(&mut items) {
        result.push(items.clone());
    }
    assert_eq!(
        result,
        [
            [1, 2, 3],
            [2, 1, 3],
            [3, 1, 2],
            [1, 3, 2],
            [2, 3, 1],
            [3, 2, 1],
        ]
    );
}

fn main() {
    test_permutation();
}
