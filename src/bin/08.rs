#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<u64>>,
    row_length: usize,
    col_length: usize
}

impl Matrix {
    fn row(&self, n: usize) -> Vec<u64> {
        let r = self.data[n].clone();
        return r;
    }

    fn col(&self, n: usize) -> Vec<u64> {
        let mut v = Vec::new();
        for x in &self.data[..] {
            for (i, y) in x.iter().enumerate() {
                if i == n {
                    v.push(*y);
                }
            }
        }
        return v;
    }

    fn left(&self, i: (usize, usize)) -> Vec<u64> {
        let (row_idx, col_idx) = i;
        let l = self.row(row_idx)[0..col_idx].to_vec();
        return l;
    }
    
    fn right(&self, i: (usize, usize)) -> Vec<u64> {
        let (row_idx, col_idx) = i;
        let r = self.row(row_idx)[col_idx+1..].to_vec();
        return r;
    }

    fn top(&self, i: (usize, usize)) -> Vec<u64> {
        let (row_idx, col_idx) = i;
        let t = self.col(col_idx)[0..row_idx].to_vec();
        return t;
    }

    fn bottom(&self, i: (usize, usize)) -> Vec<u64> {
        let (row_idx, col_idx) = i;
        return self.col(col_idx)[row_idx+1..].to_vec();
    }

    fn is_visible(&self, i: (usize, usize)) -> Option<bool> {
        let (row_idx, col_idx) = i;
        if row_idx >= self.col_length || col_idx >= self.row_length {
            return None;
        } else {
            let val = self.data[row_idx][col_idx];
            let l = self.left((row_idx, col_idx));
            if l.len() == 0 || l.iter().all(|x| x < &val) {
                return Some(true);
            } 

            let r = self.right((row_idx, col_idx));
            if r.len() == 0 || r.iter().all(|x| x < &val) {
                return Some(true);
            }

            let t = self.top((row_idx, col_idx));
            if t.len() == 0 || t.iter().all(|x| x < &val) {
                return Some(true);
            }

            let b = self.bottom((row_idx, col_idx));
            if b.len() == 0 || b.iter().all(|x| x < &val) {
                return Some(true);
            }

            return Some(false);
        }
    }

    fn scenic_score(&self, i: (usize, usize)) -> Option<u64> {
        let (row_idx, col_idx) = i;
        if row_idx >= self.col_length || col_idx >= self.row_length {
            return None;
        }
        else if row_idx == 0 || row_idx == (self.row_length - 1) || col_idx == 0 || col_idx == (self.col_length - 1) {
            return Some(0 as u64);
        } else {
            let val = self.data[row_idx][col_idx];
            let mut l_vec = self.left((row_idx, col_idx));
            l_vec.reverse();
            let r_vec = self.right((row_idx, col_idx));
            let mut t_vec = self.top((row_idx, col_idx));
            t_vec.reverse();
            let b_vec: Vec<u64> = self.bottom((row_idx, col_idx));
            let mut l_score = 0;
            let mut r_score = 0;
            let mut t_score = 0;
            let mut b_score = 0;
            for l in l_vec {
                if l >= val {
                    l_score += 1;
                    break;
                }
                l_score += 1;
            }

            for r in r_vec {
                if r >= val {
                    r_score += 1;
                    break;
                }
                r_score += 1;
            }

            for b in b_vec {
                if b >= val {
                    b_score += 1;
                    break;
                }
                b_score += 1;
            }

            for t in t_vec {
                if t >= val {
                    t_score += 1;
                    break;
                }
                t_score += 1;
            }

            let score: u64 = l_score * r_score * t_score * b_score;
            return Some(score);
        }
    }

}

fn part1(s: &String) -> u64 {
    let yeet: Vec<Vec<u64>> = s.split("\n")
        .map(|x| x.chars().map(|x| x.to_digit(10).expect("should parse char") as u64).collect()).collect();
    let c = yeet.len();
    let r = yeet[0].len();
    let m = Matrix {
        data: yeet,
        col_length: c,
        row_length: r
    };
    let mut s = 0;
    for (i, x) in m.data.iter().enumerate() {
        for (j, _) in x.iter().enumerate() {
            match m.is_visible((i, j)) {
                Some(b) => {
                    if b {
                        s += 1;
                    }
                },
                None => panic!("m.is_visible should always return a bool")
            }
        }
    }
    return s;
}

fn part2(s: &String) -> u64 {
    let yeet: Vec<Vec<u64>> = s.split("\n")
        .map(|x| x.chars().map(|x| x.to_digit(10).expect("should parse char") as u64).collect()).collect();
    let c = yeet.len();
    let r = yeet[0].len();
    let m = Matrix {
        data: yeet,
        col_length: c,
        row_length: r
    };
    let mut max = 0;
    for (i, x) in m.data.iter().enumerate() {
        for (j, _) in x.iter().enumerate() {
            match m.scenic_score((i, j)) {
                Some(b) => {
                    if b > max {
                        max = b;
                    }
                },
                None => panic!("m.is_visible should always return a bool")
            }
        }
    }
    return max;
}

fn main() -> () {
    let s = String::from(include_str!("../inputs/08_in.txt"));
    println!("{}", part1(&s));
    println!("{}", part2(&s));
}