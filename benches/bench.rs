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
        let xstr = "hello";
        let ystr = "world";
        let xdynstr1 = "hello".to_string();
        let xdynstr1 = xdynstr1.as_str();
        let xdynstr2 = "hello".to_string();
        let xdynstr2 = xdynstr2.as_str();
        let ydynstr1 = "world".to_string();
        let ydynstr1 = ydynstr1.as_str();
        let ydynstr2 = "world".to_string();
        let ydynstr2 = ydynstr2.as_str();
        c.bench_with_input(
            BenchmarkId::new("diff-arguments", format!("{},{}", x, y)),
            &(x, y),
            move |b, (x, y)| {
                b.iter(|| {
                    bench1(*x, *y);
                });
                #[inline(never)]
                fn bench1(x: u64, y: u64) {
                    let mut bump = Bump::new();
                    for _ in 0..1000 {
                        let d1 = format_diffable_args!("a{x}b{y}c{3}");
                        let d2 = format_diffable_args!("a{x}b{y}c{3}");
                        black_box(d1 == d2);
                        bump.reset()
                    }
                }
            },
        );
        c.bench_with_input(
            BenchmarkId::new("diff-arguments-dyn-str", format!("{},{}", x, y)),
            &((xdynstr1, xdynstr2), (ydynstr1, ydynstr2)),
            move |b, (x, y)| {
                b.iter(|| {
                    bench1(*x, *y);
                });
                #[inline(never)]
                fn bench1((x1, x2): (&str, &str), (y1, y2): (&str, &str)) {
                    let mut bump = Bump::new();
                    for _ in 0..1000 {
                        let d1 = format_diffable_args!("a{x1}b{y1}c{3}");
                        let d2 = format_diffable_args!("a{x2}b{y2}c{3}");
                        black_box(d1 == d2);
                        bump.reset()
                    }
                }
            },
        );
        c.bench_with_input(
            BenchmarkId::new("diff-arguments-static-str", format!("{},{}", x, y)),
            &(xstr, ystr),
            move |b, (x, y)| {
                b.iter(|| {
                    bench1(*x, *y);
                });
                #[inline(never)]
                fn bench1(x: &str, y: &str) {
                    let mut bump = Bump::new();
                    for _ in 0..1000 {
                        let d1 = format_diffable_args!("a{x}b{y}c{3}");
                        let d2 = format_diffable_args!("a{x}b{y}c{3}");
                        black_box(d1 == d2);
                        bump.reset()
                    }
                }
            },
        );
        c.bench_with_input(
            BenchmarkId::new("diff-formatted-arguments", format!("{},{}", x, y)),
            &(x, y),
            move |b, (x, y)| {
                b.iter(|| {
                    bench1(*x, *y);
                });
                #[inline(never)]
                fn bench1(x: u64, y: u64) {
                    let mut bump = Bump::new();
                    for _ in 0..1000 {
                        {
                            let x =
                                black_box(bumpalo::format!(in &bump, "{:?}", x)).into_bump_str();
                            let y =
                                black_box(bumpalo::format!(in &bump, "{:?}", y)).into_bump_str();
                            let d1 = format_diffable_args!("a{x}b{y}c{3}");
                            let d2 = format_diffable_args!("a{x}b{y}c{3}");
                            black_box(d1 == d2);
                        }
                        bump.reset()
                    }
                }
            },
        );
        c.bench_with_input(
            BenchmarkId::new("allocate-arguments", format!("{},{}", x, y)),
            &(x, y),
            move |b, (x, y)| {
                b.iter(|| {
                    let mut bump = Bump::new();
                    for _ in 0..1000 {
                        let d = format_diffable_args!("a{x}b{y}c{3}");
                        black_box(d.to_bump_str(&bump));
                        bump.reset()
                    }
                })
            },
        );
        c.bench_with_input(
            BenchmarkId::new("diff-std", format!("{},{}", x, y)),
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
            BenchmarkId::new("diff-std-dyn-str", format!("{},{}", x, y)),
            &((xdynstr1, xdynstr2), (ydynstr1, ydynstr2)),
            move |b, ((x1, x2), (y1, y2))| {
                b.iter(|| {
                    let mut bump = Bump::new();
                    for _ in 0..1000 {
                        let d1 = bumpalo::format!(in &bump, "a{}b{}c{}", x1, y1, 3).into_bump_str();
                        let d2 = bumpalo::format!(in &bump, "a{}b{}c{}", x2, y2, 3).into_bump_str();
                        black_box(d1 == d2);
                        bump.reset()
                    }
                })
            },
        );
        c.bench_with_input(
            BenchmarkId::new("diff-std-static-str", format!("{},{}", x, y)),
            &(xstr, ystr),
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
            BenchmarkId::new("diff-formatted-std", format!("{},{}", x, y)),
            &(x, y),
            move |b, (x, y)| {
                b.iter(|| {
                    bench1(*x, *y);
                });
                #[inline(never)]
                fn bench1(x: u64, y: u64) {
                    let mut bump = Bump::new();
                    for _ in 0..1000 {
                        {
                            let d1 = bumpalo::format!(in &bump, "a{x:?}b{y:?}c{}", 3);
                            let d2 = bumpalo::format!(in &bump, "a{x:?}b{y:?}c{}", 3);
                            black_box(d1 == d2);
                        }
                        bump.reset()
                    }
                }
            },
        );
        c.bench_with_input(
            BenchmarkId::new("allocate-std", format!("{},{}", x, y)),
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
