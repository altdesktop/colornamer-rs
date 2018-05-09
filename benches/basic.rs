#[macro_use]
extern crate criterion;
extern crate colornamer;

use criterion::Criterion;
use colornamer::{ColorNamer ,Colors};


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("basic", |b| b.iter(|| {
        let colornamer = ColorNamer::new(Colors::HTML);
        colornamer.name_hex_color("#1E90FF").unwrap()
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

