use aoc2020::solutions::*;
use criterion::{criterion_group, criterion_main, AxisScale, Criterion, PlotConfiguration};

fn bench_steps(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("Solutions");
    group.plot_config(plot_config);
    group.bench_function("day6 hashset", |b| b.iter(|| day6::run()));
    group.bench_function("day6 optimized", |b| b.iter(|| day6_optimized::run()));
    group.bench_function("day6 bytes", |b| b.iter(|| day6_bytes::run()));
}

criterion_group!(steps, bench_steps);
criterion_main!(steps);
