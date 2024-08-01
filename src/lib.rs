#[derive(Default)]
pub struct P2 {
    quantile: f64,

    heights: Vec<f64>,
    pos: [i64; 5],
    n_pos: [f64; 5],
    dn: [f64; 5],

    filled: bool,
}

impl P2 {
    pub fn new(quantile: f64) -> Self {
        if quantile < 0.0 || quantile > 1.0 {
            panic!("quantile must be in [0, 1]");
        }
        let mut p2 = Self {
            quantile,
            n_pos: [
                0.0,
                2.0 * quantile,
                4.0 * quantile,
                2.0 + 2.0 * quantile,
                4.0,
            ],
            dn: [0.0, quantile / 2.0, quantile, (1.0 + quantile) / 2.0, 1.0],
            ..Default::default()
        };

        for i in 0..p2.pos.len() {
            p2.pos[i] = i as i64;
        }

        p2
    }

    pub fn append(&mut self, data: f64) {
        if self.heights.len() != 5 {
            self.heights.push(data);
            return;
        }
        if !self.filled {
            self.filled = true;
            self.heights.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
        self.append_data(data);
    }

    fn append_data(&mut self, data: f64) {
        let l = self.heights.len() - 1;
        let mut k: isize = -1;
        if data < self.heights[0] {
            k = 0;
            self.heights[0] = data;
        } else if self.heights[l] <= data {
            k = l as isize - 1;
            self.heights[l] = data;
        } else {
            for i in 1..=l {
                if self.heights[i - 1] <= data && data < self.heights[i] {
                    k = i as isize - 1;
                    break;
                }
            }
        }
        for i in 0..self.pos.len() {
            if i > k as usize {
                self.pos[i] += 1;
            }
            self.n_pos[i] += self.dn[i];
        }

        self.adjust_heights();
    }

    fn adjust_heights(&mut self) {
        for i in 1..self.heights.len() - 1 {
            let n = self.pos[i];
            let np1 = self.pos[i + 1];
            let nm1 = self.pos[i - 1];

            let d = self.n_pos[i] - n as f64;

            if (d >= 1.0 && np1 - n > 1) || (d <= -1.0 && nm1 - n < -1) {
                let d = if d >= 0.0 { 1.0 } else { -1.0 };

                let h = self.heights[i];
                let hp1 = self.heights[i + 1];
                let hm1 = self.heights[i - 1];

                let hi = parabolic(d, hp1, h, hm1, np1 as f64, n as f64, nm1 as f64);

                if hm1 < hi && hi < hp1 {
                    self.heights[i] = hi;
                } else {
                    let hd = self.heights[i + d as usize];
                    let nd: i64 = self.pos[i + d as usize];
                    self.heights[i] = h + d * (hd - h) / (nd - n) as f64;
                }

                self.pos[i] += d as i64;
            }
        }
    }

    pub fn value(&mut self) -> f64 {
        if !self.filled {
            let l = self.heights.len();
            match l {
                0 => return 0.0,
                1 => return self.heights[0],
                _ => self.heights.sort_by(|a, b| a.partial_cmp(b).unwrap()),
            }
            let rank = (self.quantile * l as f64) as usize;
            return self.heights[rank.min(l - 1)];
        }
        self.heights[2]
    }
}

fn parabolic(d: f64, qp1: f64, q: f64, qm1: f64, np1: f64, n: f64, nm1: f64) -> f64 {
    let a = d / (np1 - nm1);
    let b1 = (n - nm1 + d) * (qp1 - q) / (np1 - n);
    let b2 = (np1 - n - d) * (q - qm1) / (n - nm1);
    q + a * (b1 + b2)
}

#[cfg(test)]
mod test {
    use super::P2;

    #[test]
    fn test_p2() {
        let mut p2 = P2::new(0.3);

        for i in 1..=100 {
            p2.append(i as f64);
        }

        let x = p2.value();
        assert_eq!(x, 30.0);
    }
}
