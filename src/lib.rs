struct EWMA {
    val: f64,
    alpha: f64,
}

#[allow(dead_code)]
impl EWMA {
    pub fn new(alpha: f64) -> EWMA {
        EWMA{ val: 0.0, alpha: alpha }
    }

    pub fn value(&self) -> f64 {
        return self.val;
    }

    pub fn add(&mut self, new_val: f64) -> f64 {
        self.val = self.alpha*new_val + (1.0-self.alpha)*self.val;
        return self.val;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ewma() {
        let mut ewma = ::EWMA::new(0.1);
        assert_eq!(ewma.value(), 0.0);
        assert_eq!(ewma.add(0.5), 0.05);
    }
}
