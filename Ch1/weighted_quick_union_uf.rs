use std::io::stdin;

struct WeightedQuickUnion {
    id: Vec<usize>,
    sz: Vec<usize>,
    count: usize,
}

impl WeightedQuickUnion {
    fn new(n: usize) -> Self {
        Self {
            id: (0..n).into_iter().collect(),
            sz: vec![1; n],
            count: n,
        }
    }

    fn union(&mut self, p: usize, q: usize) {
        assert!(p < self.count);
        assert!(q < self.count);

        let p_root: usize = self.find(p);
        let q_root: usize = self.find(q);
        if p_root == q_root {
            return;
        }

        if self.sz[p_root] > self.sz[q_root] {
            self.id[q_root] = p_root;
            self.sz[p_root] += self.sz[q_root];
        } else {
            self.id[p_root] = q_root;
            self.sz[q_root] += self.sz[p_root];
        }
        self.count -= 1;
    }

    fn find(&self, mut p: usize) -> usize {
        assert!(p < self.count);

        while self.id[p] != p {
            p = self.id[p];
        }

        p
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        assert!(p < self.count);
        assert!(q < self.count);
        self.find(p) == self.find(q)
    }

    fn count(&self) -> usize {
        self.count
    }
}
fn main() {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).unwrap();
    let mut uf: WeightedQuickUnion = WeightedQuickUnion::new(buf.trim().parse().unwrap());
    buf.clear();

    stdin()
        .lines()
        .map(|res_line: Result<String, std::io::Error>| res_line.unwrap())
        .for_each(|line: String| {
            let v: Vec<usize> = line
                .split_whitespace()
                .map(|str_num: &str| str_num.parse::<usize>().unwrap())
                .collect();

            if !uf.connected(v[0], v[1]) {
                uf.union(v[0], v[1]);
            }
        });

    println!("{} components", uf.count())
}

