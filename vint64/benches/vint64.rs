//! vint64 benchmark (using criterion)

use criterion::{criterion_group, criterion_main, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;

const EXAMPLE_VALUES: [u64; 8] = [
    0,
    0x0f,
    0x0f0f,
    0x0f0f_f0f0,
    0x0f0f_f0f0_0f0f,
    0x0f0f_f0f0_0f0f_f0f0,
    0xffff_ffff_0f0f_f0f0,
    core::u64::MAX,
];

fn bench(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("vint64");

    group.bench_function("encode", |b| {
        let mut n = 0;
        b.iter(|| {
            vint64::encode(EXAMPLE_VALUES[n]);
            n = (n + 1) & 0x07;
        });
    });

    group.bench_function("decode", |b| {
        let examples = [
            vint64::encode(EXAMPLE_VALUES[0]),
            vint64::encode(EXAMPLE_VALUES[1]),
            vint64::encode(EXAMPLE_VALUES[2]),
            vint64::encode(EXAMPLE_VALUES[3]),
            vint64::encode(EXAMPLE_VALUES[4]),
            vint64::encode(EXAMPLE_VALUES[5]),
            vint64::encode(EXAMPLE_VALUES[6]),
            vint64::encode(EXAMPLE_VALUES[7]),
        ];

        let mut n = 0;

        b.iter(|| {
            let mut slice = examples[n].as_ref();
            vint64::decode(&mut slice).unwrap();
            n = (n + 1) & 0x07;
        });
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = bench
);

criterion_main!(benches);
