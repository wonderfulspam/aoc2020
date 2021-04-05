use aoc2020::solutions::*;
use criterion::{criterion_group, criterion_main, AxisScale, Criterion, PlotConfiguration};

fn bench_steps(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("Solutions");
    group.plot_config(plot_config);
    group.bench_function("day01", |b| b.iter(day1_iterators_smart::run));
    group.bench_function("day02", |b| b.iter(day2_stringsplit::run));
    group.bench_function("day03", |b| b.iter(day3::run));
    group.bench_function("day04", |b| b.iter(day4::run));
    group.bench_function("day05", |b| b.iter(day5::run));
    group.bench_function("day06", |b| b.iter(day6_bytes::run));
    group.bench_function("day07", |b| b.iter(day7::run));
    group.bench_function("day08", |b| b.iter(day8::run));
    group.bench_function("day09", |b| b.iter(day9::run));
    group.bench_function("day10", |b| b.iter(day10::run));
    group.bench_function("day11", |b| b.iter(day11::run));
    group.bench_function("day12", |b| b.iter(day12::run));
    group.bench_function("day13", |b| b.iter(day13::run));
    group.bench_function("day14", |b| b.iter(day14::run));
    group.bench_function("day15", |b| b.iter(day15::run));
    group.bench_function("day18", |b| b.iter(day15::run));
}

criterion_group!(steps, bench_steps);
criterion_main!(steps);
