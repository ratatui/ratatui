use rand::Rng;

/// Generates realistic implied volatility surface data with animation
pub struct VolatilityEngine {
    strikes: Vec<f64>,      // Moneyness (0.8 to 1.2)
    expirations: Vec<f64>,  // Time to expiration in years
    surface: Vec<Vec<f64>>, // IV surface [expiry][strike]
    base_vol: f64,
    skew: f64,
    term_structure: Vec<f64>,
    time: f64,
}

impl VolatilityEngine {
    pub fn new() -> Self {
        let strikes: Vec<f64> = (0..25).map(|i| 0.7 + f64::from(i) * 0.025).collect(); // 70% to 130%
        let expirations: Vec<f64> = (0..20).map(|i| 0.02 + f64::from(i) * 0.1).collect(); // 1W to 2Y

        let mut engine = Self {
            strikes,
            expirations,
            surface: Vec::new(),
            base_vol: 20.0,
            skew: 0.3,
            term_structure: Vec::new(),
            time: 0.0,
        };

        engine.initialize();
        engine
    }

    fn initialize(&mut self) {
        // Generate term structure (volatility term structure)
        self.term_structure = self
            .expirations
            .iter()
            .map(|&t| {
                // Typical volatility term structure with contango/backwardation
                let base = self.base_vol;
                let term_effect = 5.0 * (1.0 - (-t * 2.0).exp());
                base + term_effect
            })
            .collect();

        // Generate full surface
        self.regenerate_surface(0.0);
    }

    pub fn update(&mut self) {
        self.time += 0.05;
        self.regenerate_surface(self.time);
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
        self.regenerate_surface(self.time);
    }

    fn regenerate_surface(&mut self, time: f64) {
        let mut rng = rand::rng();
        self.surface.clear();

        for (exp_idx, &expiry) in self.expirations.iter().enumerate() {
            let mut row = Vec::new();
            let term_vol = self.term_structure[exp_idx];

            // Add some time-based variation
            let time_wave = (time * 0.5 + exp_idx as f64 * 0.1).sin() * 2.0;
            let vol_shock = (time * 0.3).sin() * 1.5; // Market-wide vol shock

            for &strike in &self.strikes {
                let moneyness = strike;

                // Classic volatility smile/skew model
                let log_moneyness = (moneyness).ln();

                // Skew component (negative for put skew)
                let skew_component = -self.skew * log_moneyness * 100.0 / expiry.sqrt();

                // Smile component (convexity)
                let smile_component = 5.0 * log_moneyness.powi(2) / expiry.sqrt();

                // Wings (out-of-the-money options have higher vol)
                let wing_component = if (0.95..=1.05).contains(&moneyness) {
                    0.0
                } else {
                    ((moneyness - 1.0).abs() - 0.05) * 20.0
                };

                // Realized volatility clustering effect
                let cluster = (time * 2.0 + strike * 10.0).sin().abs() * 1.5;

                // Random noise
                let noise = (rng.random::<f64>() - 0.5) * 0.5;

                let iv = term_vol
                    + skew_component
                    + smile_component
                    + wing_component
                    + time_wave
                    + vol_shock
                    + cluster
                    + noise;

                row.push(iv.clamp(5.0, 80.0)); // Clamp to reasonable range
            }
            self.surface.push(row);
        }
    }

    pub const fn get_surface(&self) -> &Vec<Vec<f64>> {
        &self.surface
    }
}
