use std::time::Instant;
use crate::parabolic_sar::ParabolicSAR;

pub fn measure_performance(sar: &mut ParabolicSAR, new_candle: (f64, f64)) {
    let start_time = Instant::now();

    sar.update(new_candle.0, new_candle.1);

    let elapsed = start_time.elapsed();
    println!(
        "SAR updated in {:?} nanoseconds. New SAR Value: {:.6}",
        elapsed.as_nanos(),
        sar.get_sar()
    );
}
