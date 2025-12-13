use std::str::FromStr;

pub fn read_lines(day: u8) -> Vec<String> {
    let input = std::fs::read_to_string(format!("input/day{day:02}.txt"))
        .expect("Failed to read input file");
    input.lines().map(String::from).collect()
}

#[derive(Debug, Copy, Clone)]
pub struct Range(pub u64, pub u64);

impl Range {
    pub fn sum_repeats<F>(&self, f: F) -> u64
    where
        F: Fn(u64) -> bool,
    {
        (self.0..=self.1).filter(|&x| f(x)).sum()
    }

    pub fn contains_inclusive(&self, n: u64) -> bool {
        self.0 <= n && n <= self.1
    }

    pub fn cardinality(&self) -> u64 {
        (self.1 - self.0) + 1
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.0 <= other.1 && other.0 <= self.1
    }

    pub fn merge(&mut self, other: &Self) {
        self.0 = self.0.min(other.0);
        self.1 = self.1.max(other.1);
    }
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once('-').ok_or("No delimiter")?;
        let start: u64 = l.trim().parse().map_err(|_| "Invalid start")?;
        let end: u64 = r.trim().parse().map_err(|_| "Invalid end")?;
        Ok(Self(start, end))
    }
}

#[macro_export]
macro_rules! timed {
    ($label:expr, $expr:expr) => {{
        let now = std::time::Instant::now();
        let result = $expr;
        let elapsed = now.elapsed();
        println!(
            "{}: {} ({}ms)",
            $label,
            result,
            elapsed.as_micros() as f64 / 1000.0
        );
        result
    }};
}
