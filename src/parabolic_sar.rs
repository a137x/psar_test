pub struct ParabolicSAR {
    sar: f64,
    af: f64,
    ep: f64,
    is_uptrend: bool,
}

impl ParabolicSAR {
    pub fn new(candles: &[(f64, f64)]) -> Self {
        let (first_low, _first_high) = candles[0];
        let (_last_low, last_high) = candles[candles.len() - 1];

        let initial_sar = first_low;  // Start SAR at first low
        let initial_ep = last_high;   // Extreme Point is last high
        let initial_af = 0.02;        // Initial Acceleration Factor

        Self {
            sar: initial_sar,
            af: initial_af,
            ep: initial_ep,
            is_uptrend: true,
        }
    }

    pub fn update(&mut self, low: f64, high: f64) {
        // Update EP (Extreme Point)
        if self.is_uptrend && high > self.ep {
            self.ep = high;
            self.af = (self.af + 0.02).min(0.20);
        } else if !self.is_uptrend && low < self.ep {
            self.ep = low;
            self.af = (self.af + 0.02).min(0.20);
        }

        // Compute new SAR
        let new_sar = self.sar + self.af * (self.ep - self.sar);

        // Trend reversal check
        if self.is_uptrend && new_sar > low {
            self.is_uptrend = false;
            self.sar = self.ep;
            self.af = 0.02;
        } else if !self.is_uptrend && new_sar < high {
            self.is_uptrend = true;
            self.sar = self.ep;
            self.af = 0.02;
        } else {
            self.sar = new_sar;
        }
    }

    pub fn get_sar(&self) -> f64 {
        self.sar
    }
}
