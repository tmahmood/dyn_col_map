# Column Mapper

**this name may not be final**

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

This also provides benefit with different datasets, which may not have similar columns.
So, if you have one dataset with columns `c1` and `c2` another with `c5` and `c6`, all you have to do is following

```rust
fn main() {
    let mut cm = ColumnMapper::new();
    // first dataset, but you can add all of the columns beforehand as usual
    // cm.add_columns(vec!["c0", "c1", "c4", "c5"]);

    cm.add_columns(vec!["c0", "c1"]);
    let mut row = Vec::new();
    // insert data for first dataset
    row.push(vec![]);
    cl! {
            ins cm, row[0],
            kv "c0", "c0v",
            kv "c1", "Something"
        }
    // now another dataset found
    cm.add_columns(vec!["c4", "c5"]);

    // insert data for second dataset
    row.push(vec![]);
    cl! {
            ins cm, row[1],
            kv "c4", "v2",
            kv "c5", "32"
        }

    // another dataset with mixed columns, as names are common, 
    // no new columns will be added and the sequence will stay 
    // the same
    cm.add_columns(vec!["c1", "c5"]);
    row.push(vec![]);
    cl! {
            ins cm, row[2],
            kv "c1", "another set",
            kv "c5", "mixed dataset"
        }
    assert_eq!(
        row,
        vec![
            // NOTE: this is not filled up. how to handle it is discussed next
            vec!["c0v", "Something"],
            vec!["", "", "v2", "32"],
            vec!["", "another set", "", "mixed dataset"],
        ]
    );
}
```

One issue is, as noted in the example above, any rows before a new column is added, will fail, as the column does not exist on them. Any rows added after will have them. To solve this issue, `fill_to_end` method should be used for each row as necessary. 

Following example provided to clear the issue and solution.
 
```rust
    fn main() {
    let mut cm = ColumnMapper::new();
    cm.add_columns(vec!["c0", "c1", "c2", "c3"]);
    let mut rows = Vec::new();

    rows.push(vec![]);
    cl! {
            ins cm, rows[0],
            kv "c0", "r1d0",
            kv "c2", "r1d2"
        }

    // now a new column is added
    cm.add_column("c4");

    // this will cause a NoDataSet error, cause column c4 was created after setting this row, and it does not exists
    let n = cm.get(&rows[0], "c4");
    assert!(n.is_err());

    // fill the row with
    cm.fill_to_end(&mut rows[0]);
    // now it will be okay
    let n = cm.get(&rows[0], "c4");
    assert!(n.is_ok());

    // all the next rows will have all the columns
    rows.push(vec![]);
    cl! {
            ins cm, rows[1],
            kv "c0", "r2d0",
            kv "c2", "r2d2"
        }

    // this will work without filling up
    let n = cm.get(&rows[1], "c4");
    assert!(n.is_ok());

    println!("{:?}", rows);
}
```

Here's how to save data to a CSV file using `csv` crate

```rust

pub fn write_to_csv(
    file_name: PathBuf,
    rows: &Vec<Vec<String>>,
    col_mapper: &ColumnMapper,
) {
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .from_path(file_name)
        .unwrap();
    let all_columns = col_mapper.get_columns();
    writer.write_record(&all_columns).unwrap();
    for row in rows.iter() {
        if row.len() != all_columns.len() {
            let mut new_row = row.clone();
            col_mapper.fill_to_end(&mut new_row);
            writer.write_record(new_row).unwrap()
        } else {
            writer.write_record(row).unwrap()
        }
    }
    writer.flush().unwrap();
}
```

## What this crate tries to solve?

It is trying to maintain the lower memory usage of a vec and ordered key based accessing of an IndexMap.