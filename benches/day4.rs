use aoc2020::solutions::*;
use criterion::{criterion_group, criterion_main, AxisScale, Criterion, PlotConfiguration};

fn bench_steps(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("Solutions");
    group.plot_config(plot_config);
    group.bench_function("day4 regex", |b| b.iter(|| day4::run()));
    group.bench_function("day4 inlined", |b| b.iter(|| day4_inlined::run()));
}

criterion_group!(steps, bench_steps);
criterion_main!(steps);
