use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::distributions::{Alphanumeric, DistString};
use std::collections::{BTreeMap, HashMap};

use dyn_col_map::table_map::TableMap;
use dyn_col_map::{push, table_map, update_row};

macro_rules! inserter {
    ($($c: expr, $v: stmt),+) => {
        $c, $v,+
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut tm = TableMap::new();
    for _ in 0..100 {
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        tm.add_column(&string)
    }
    let columns = tm.get_columns();
    let mut group = c.benchmark_group("insertion bench");
    group.bench_function("TableMap", |b| {
        b.iter(|| {
            for _ in 0..100 {
                for ii in 0..100 {
                    update_row! {
                        tm,
                        &columns[ii],
                        Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
                    }
                }
                tm.next_row();
            }
        })
    });
    group.bench_function("inserting hash map", |b| {
        b.iter(|| {
            let mut n: Vec<Vec<String>> = vec![];
            for _ in 0..100 {
                let mut hm = BTreeMap::new();
                for ii in 0..100 {
                    hm.insert(
                        columns[ii].as_str(),
                        Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
                    );
                }
                n.push(hm.values().cloned().collect())
            }
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
