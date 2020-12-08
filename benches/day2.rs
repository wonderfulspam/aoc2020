use aoc2020::solutions::*;
use criterion::{criterion_group, criterion_main, AxisScale, Criterion, PlotConfiguration};

fn bench_steps(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("Solutions");
    group.plot_config(plot_config);
    group.bench_function("day2 parse_display", |b| b.iter(|| day2::run()));
    group.bench_function("day1 string split", |b| b.iter(|| day2_stringsplit::run()));
}

criterion_group!(steps, bench_steps);
criterion_main!(steps);
