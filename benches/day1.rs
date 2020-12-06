use aoc2020::solutions::*;
use criterion::{criterion_group, criterion_main, AxisScale, Criterion, PlotConfiguration};

fn bench_steps(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("Solutions");
    group.plot_config(plot_config);
    group.bench_function("day1 combinations", |b| b.iter(|| day1::run()));
    group.bench_function("day1 iterators", |b| b.iter(|| day1_iterators::run()));
}

criterion_group!(steps, bench_steps);
criterion_main!(steps);
