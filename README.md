# TableMap

**this name may not be final**

HashMap, BTreeMap, IndexMap needs a lot of memory in case of String based keys, and large number of data.

This is a simple library that tries to memory efficiently provide a `IndexMap` with a String key like functionality using vecs, that might have a large number of data with string keys. There might be other better solutions in the wild.

As the String keys are mapped to vec index we are storing the string keys only once, I have not benchmarked it, so can not say anything about performance, yet.

Simple macros are provided for easy assigning of data.

#### To do

- [ ] fill to end row by index
- [ ] Removing a row
- [ ] Clear data


## Initializing and inserting

```rust
use dyn_col_map::table_map::TableMap;
use dyn_col_map::{push, update_row};

fn main() {
    let mut cm = TableMap::new();
    cm.add_columns(vec!["c0", "c1", "c2", "c3"]);
    // single insert, no new row added
    cm.insert("c1", "Something").unwrap();
    // single insert using macro, no new row added
    update_row! { cm, "c0", "c0v" }
    // multiple inserts using macro, will not add new row
    update_row! {
        cm,
        "c1", "Something",
        "c2", "v2",
        "c3", "32"
    }
    // this will create a new row and insert
    push! {
        cm,
        "c0", "Another thing",
        "c1", "second column",
        "c2", "another value",
        "c3", "final column"
    }
    // getting a value from current row
    let v = cm.get_column_value("c1").unwrap();
    assert_eq!(v, "second column");
    // getting a value from another row
    let v = cm.get_column_value_by_index(0, "c1").unwrap();
    assert_eq!(v, "Something");
}
```

This also provides benefit with different datasets, which may not have similar columns.
So, in case of one dataset with columns `c1` and `c2` another with `c5` and `c6`

```rust
use dyn_col_map::{push, update_row};
use dyn_col_map::table_map::TableMap;
fn main() {
    let mut cm = TableMap::new();
    // first dataset, but you can add all of the columns beforehand as usual
    // cm.add_columns(vec!["c0", "c1", "c4", "c5"]);

    cm.add_columns(vec!["c0", "c1"]);
    // insert data for first dataset
    push! {
        cm,
        "c0", "c0v",
        "c1", "Something"
    }
    // now another dataset found
    cm.add_columns(vec!["c4", "c5"]);
    // insert data for second dataset
    push! {
        cm,
        "c4", "v2",
        "c5", "32"
    }

    // another dataset with mixed columns, as names are common,
    // no new columns will be added and the sequence will stay
    // the same
    cm.add_columns(vec!["c1", "c5"]);
    push! {
        cm,
        "c1", "another set",
        "c5", "mixed dataset"
    }
    assert_eq!(
        cm.get_vec(),
        &vec![
            vec!["c0v", "Something"],  // NOTE: this is not filled up
            vec!["", "", "v2", "32"],
            vec!["", "another set", "", "mixed dataset"],
        ]
    );
}

```

One issue is, as noted in the example above, any rows inserted before a new column is added,
will not be filled up, and cause error when we try to get value for the new column from those
rows. Any rows added after will have them.

To solve this issue, `fill_to_end` method should be used for each row as necessary.

Following example attempts to clarify the issue, and provide solution.

```rust
    use dyn_col_map::{push, update_row};
    use dyn_col_map::table_map::TableMap;
    fn main() {
        let mut cm = TableMap::new();
        cm.add_columns(vec!["c0", "c1", "c2", "c3"]);

        update_row! {
            cm,
            "c0", "r1d0",
            "c2", "r1d2"
        }

        // now a new column is added
        cm.add_column("c4");

        // this will cause a NoDataSet error, cause column c4 was created after setting
        // this row, and it does not exists
        let n = cm.get_column_value("c4");
        assert!(n.is_err());

        // fill the row with default value
        cm.fill_to_end();
        // now it will be okay
        let n = cm.get_column_value("c4");
        assert!(n.is_ok());

        // all the next rows will have all the columns
        push! {
            cm,
            "c0", "r2d0",
            "c2", "r2d2"
        }

        // this will work without filling up
        let n = cm.get_column_value("c4");
        assert!(n.is_ok());
    }

```

## What this crate tries to solve?

It is trying to maintain the lower memory usage of a vec and ordered key based accessing of an IndexMap.

In my own testing, with a dataset of 947300 rows,
* HashMap/IndexMap implementation was out of memory on my 64GB machine,
* TableMap was 37GB.
* Interestingly Python was only 27GB.

As I understand, HashMap/IndexMap, stores all the keys for each row, and in addition to that, they provide performance for the price of high memory usage. Unfortunately, It was not suitable for my task and I have not found any other solutions online. So here's what I devised.

`fill_to_end` may not be optimal. If I ever find a better way, I will try to incorporate it.

