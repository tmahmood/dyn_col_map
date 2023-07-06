# Column Mapper

HashMap, BTreeMap, IndexMap needs a lot of memory in case of String based keys and large number of data.

This is a simple library that tries to memory efficiently provide a `IndexMap` like functionality, that might have a large number of data with string keys, using vecs. There might be other better solutions in the wild. In my own testing, I was able to reduce memory usage more than half with a dataset of 947300 rows. As vec index are mapped with String based keys, we keep the best of both worlds. I have not benchmarked it, so can not say anything about performance.

Simple macros are provided to easy assigning of data.

```rust

fn main() {
    let mut cm = ColumnMapper::new();
    cm.add_columns(vec!["c0", "c1", "c2", "c3"]);

    let mut row = vec![];
    
    // single insert
    cm.insert(&mut row, "c1", "Something").unwrap();

    // single insert using macro
    cl! { ins cm, row, "c0", "c0v" }

    // multiple inserts using macro
    cl! {
        ins cm, row,
        kv "c1", "Something",
        kv "c2", "v2",
        kv "c3", "32"
    }
    
    // getting a value
    let v = cm.get(&row, "c1").unwrap();
    assert_eq!(v, "Something")
}
```


## What this crate tries to solve?

It is trying to maintain the lower memory usage of a vec and ordered key based accessing of an IndexMap.