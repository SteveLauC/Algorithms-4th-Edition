use std::io::stdin;
struct UF {
    id: Vec<usize>,
    count: usize,
}

impl UF {
    fn new(n: usize) -> Self {
        Self {
            id: (0..n).into_iter().collect(),
            count: n,
        }
    }

    fn union(&mut self, p: usize, q: usize) {
        assert!(p < self.count);
        assert!(q < self.count);

        let p_id: usize = self.find(p);
        let q_id: usize = self.find(q);
        if p_id == q_id {
            return;
        }

        self.id[p_id] = q_id;
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
    let mut uf: UF = UF::new(buf.trim().parse().unwrap());
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
