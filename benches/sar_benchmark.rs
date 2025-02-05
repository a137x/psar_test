use criterion::{black_box, criterion_group, criterion_main, Criterion};
use parabolic_sar::ParabolicSAR;

fn bench_sar_update(c: &mut Criterion) {
    let candles = vec![
        // 50 candles, only keeping highs and lows
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

    let mut sar = ParabolicSAR::new(&candles);
    let new_candle = (1.4, 1.6);

    c.bench_function("SAR Update", |b| {
        b.iter(|| {
            sar.update(black_box(new_candle.0), black_box(new_candle.1));
        })
    });
}

criterion_group!(benches, bench_sar_update);
criterion_main!(benches);
