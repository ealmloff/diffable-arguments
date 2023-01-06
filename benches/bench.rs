use std::{fmt::format, hint::black_box};

use bumpalo::Bump;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use diffable_arguments::*;

criterion_group!(mbenches, create_rows);
criterion_main!(mbenches);

fn create_rows(c: &mut Criterion) {
    for x in [0u64, 100, 1000, 10000] {
        let x = black_box(x);
        let y = black_box(x);
        c.bench_with_input(
            BenchmarkId::new("diff", format!("{},{}", x, y)),
            &(x, y),
            move |b, (x, y)| {
                b.iter(|| {
                    bench1(*x, *y);
                });
                #[inline(never)]
                fn bench1(x: u64, y: u64) {
                    let mut bump = Bump::new();
                    let static_segments = &["a", "b", "c", ""];
                    for _ in 0..1000 {
                        let d1 = DiffableArguments {
                            static_segments,
                            dynamic_segments: bump.alloc_with(|| {
                                [
                                    (&mut &x).into_entry(&bump),
                                    (&mut &y).into_entry(&bump),
                                    (&mut &3u64).into_entry(&bump),
                                ]
                            }),
                        };
                        let d2 = DiffableArguments {
                            static_segments,
                            dynamic_segments: bump.alloc_with(|| {
                                [
                                    (&mut &x).into_entry(&bump),
                                    (&mut &y).into_entry(&bump),
                                    (&mut &3u64).into_entry(&bump),
                                ]
                            }),
                        };
                        black_box(d1 == d2);
                        bump.reset()
                    }
                }
            },
        );
        c.bench_with_input(
            BenchmarkId::new("diff-formatted", format!("{},{}", x, y)),
            &(x, y),
            move |b, (x, y)| {
                b.iter(|| {
                    bench1(*x, *y);
                });
                #[inline(never)]
                fn bench1(x: u64, y: u64) {
                    let mut bump = Bump::new();
                    let static_segments = &["a", "b", "c", ""];
                    for _ in 0..1000 {
                        {
                            let x =
                                black_box(bumpalo::format!(in &bump, "{:?}", x)).into_bump_str();
                            let y =
                                black_box(bumpalo::format!(in &bump, "{:?}", y)).into_bump_str();
                            let d1 = DiffableArguments {
                                static_segments,
                                dynamic_segments: bump.alloc_with(|| {
                                    [
                                        (&mut &x).into_entry(&bump),
                                        (&mut &y).into_entry(&bump),
                                        (&mut &3u64).into_entry(&bump),
                                    ]
                                }),
                            };
                            let d2 = DiffableArguments {
                                static_segments,
                                dynamic_segments: bump.alloc_with(|| {
                                    [
                                        (&mut &x).into_entry(&bump),
                                        (&mut &y).into_entry(&bump),
                                        (&mut &3u64).into_entry(&bump),
                                    ]
                                }),
                            };
                            black_box(d1 == d2);
                        }
                        bump.reset()
                    }
                }
            },
        );
        c.bench_with_input(
            BenchmarkId::new("allocate", format!("{},{}", x, y)),
            &(x, y),
            move |b, (x, y)| {
                b.iter(|| {
                    let mut bump = Bump::new();
                    for _ in 0..1000 {
                        let d = DiffableArguments {
                            static_segments: &["a", "b", "c", ""],
                            dynamic_segments: bump.alloc_with(|| {
                                [
                                    (&mut &x).into_entry(&bump),
                                    (&mut &y).into_entry(&bump),
                                    (&mut &3u64).into_entry(&bump),
                                ]
                            }),
                        };
                        black_box(d.to_bump_str(&bump));
                        bump.reset()
                    }
                })
            },
        );
        c.bench_with_input(
            BenchmarkId::new("diff2", format!("{},{}", x, y)),
            &(x, y),
            move |b, (x, y)| {
                b.iter(|| {
                    let mut bump = Bump::new();
                    for _ in 0..1000 {
                        let d1 = bumpalo::format!(in &bump, "a{}b{}c{}", x, y, 3).into_bump_str();
                        let d2 = bumpalo::format!(in &bump, "a{}b{}c{}", x, y, 3).into_bump_str();
                        black_box(d1 == d2);
                        bump.reset()
                    }
                })
            },
        );
        c.bench_with_input(
            BenchmarkId::new("allocate2", format!("{},{}", x, y)),
            &(x, y),
            move |b, (x, y)| {
                b.iter(|| {
                    let mut bump = Bump::new();
                    for _ in 0..1000 {
                        black_box(bumpalo::format!(in &bump, "a{}b{}c{}", x, y, 3));
                        bump.reset()
                    }
                })
            },
        );
    }
}
