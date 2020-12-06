use aoc2020::solutions::*;
use criterion::{criterion_group, criterion_main, AxisScale, Criterion, PlotConfiguration};

fn bench_steps(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("Solutions");
    group.plot_config(plot_config);
    group.bench_function("day1", |b| b.iter(|| day1_dumb::run()));
    group.bench_function("day2", |b| b.iter(|| day2::run()));
    group.bench_function("day3", |b| b.iter(|| day3::run()));
    group.bench_function("day4", |b| b.iter(|| day4::run()));
    group.bench_function("day5", |b| b.iter(|| day5::run()));
    group.bench_function("day6", |b| b.iter(|| day6::run()));
}

criterion_group!(steps, bench_steps);
criterion_main!(steps);
