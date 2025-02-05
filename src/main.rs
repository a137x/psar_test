mod parabolic_sar;
mod benchmark;

use parabolic_sar::ParabolicSAR;
use benchmark::measure_performance;

fn main() {
    // Simulated initial dataset (50 candles)
    let candles = vec![
        (1.0, 1.2), (1.1, 1.3), (1.2, 1.4), (1.3, 1.5), (1.4, 1.6),
        (1.5, 1.7), (1.6, 1.8), (1.7, 1.9), (1.8, 2.0), (1.9, 2.1),
        (2.0, 2.2), (2.1, 2.3), (2.2, 2.4), (2.3, 2.5), (2.4, 2.6),
        (2.5, 2.7), (2.6, 2.8), (2.7, 2.9), (2.8, 3.0), (2.9, 3.1),
        (3.0, 3.2), (3.1, 3.3), (3.2, 3.4), (3.3, 3.5), (3.4, 3.6),
        (3.5, 3.7), (3.6, 3.8), (3.7, 3.9), (3.8, 4.0), (3.9, 4.1),
        (4.0, 4.2), (4.1, 4.3), (4.2, 4.4), (4.3, 4.5), (4.4, 4.6),
        (4.5, 4.7), (4.6, 4.8), (4.7, 4.9), (4.8, 5.0), (4.9, 5.1),
        (5.0, 5.2), (5.1, 5.3), (5.2, 5.4), (5.3, 5.5), (5.4, 5.6),
        (5.5, 5.7), (5.6, 5.8), (5.7, 5.9), (5.8, 6.0), (5.9, 6.1),
    ];

    // Initialize Parabolic SAR with the first 50 candles
    let mut sar = ParabolicSAR::new(&candles);

    // Simulated new candle input
    let new_candle = (1.4, 1.6);
    
    // Measure performance of SAR update on new candle
    measure_performance(&mut sar, new_candle);
}
