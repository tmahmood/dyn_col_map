pub mod column_map;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::column_map::colum_map_errors::ColumnMapErrors;
    use crate::column_map::ColumnMap;

    #[test]
    fn test_col_mapper_macro() {
        let mut cm = ColumnMap::new();
        cm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        let mut row = vec![];
        cl! { ins cm, row, "c0", "c0v" }
        cl! {
            ins cm, row,
            kv "c1", "Something",
            kv "c2", "v2",
            kv "c3", "32"
        }
        // get all the columns, sequence is maintained
        assert_eq!(cm.get_columns(), vec!["c0", "c1", "c2", "c3"]);
        assert_eq!(row, vec!["c0v", "Something", "v2", "32"])
    }

    #[test]
    fn test_col_mapper_macro_obj() {
        #[derive(Clone, Default, PartialEq, Debug)]
        struct TestStruct {
            val: i32,
        }
        let ar = vec![
            TestStruct { val: 30 },
            TestStruct { val: 100 },
            TestStruct { val: 1230 },
            TestStruct { val: 800 },
        ];
        let mut column_mapper = ColumnMap::new();
        column_mapper.add_columns(vec!["c0", "c1", "c2", "c3"]);
        let mut row = vec![];
        cl! { ins column_mapper, row, "c0", ar[0].clone() }
        cl! {
            ins column_mapper, row,
            kv "c1", ar[1].clone(),
            kv "c2", ar[2].clone(),
            kv "c3", ar[3].clone()
        }
        assert_eq!(column_mapper.get_columns(), vec!["c0", "c1", "c2", "c3"]);
        assert_eq!(row, ar)
    }

    #[test]
    fn test_insert_randomly() {
        let mut cm = ColumnMap::new();
        cm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        let mut row = Vec::new();

        cl! {
            ins cm, row,
            kv "c1", "Something",
            kv "c3", "Another thing",
            kv "c2", "First thing"
        }
        assert_eq!(cm.get(&row, "c1").unwrap(), "Something");
        assert!(cm.get(&row, "c10").is_err());
        assert_eq!(row, vec!["", "Something", "First thing", "Another thing"]);
    }

    #[test]
    fn test_extending_with_new_column() {
        let mut cm = ColumnMap::new();
        cm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        let mut row = Vec::new();
        cl! {
            ins cm, row,
            kv "c1", "Something",
            kv "c3", "Another thing",
            kv "c2", "First thing"
        }
        cm.add_column("c5");
        cm.insert(&mut row, "c0", "First First thing").unwrap();

        // no matter how the data is inserted, the sequence of column is maintained
        assert_eq!(
            row,
            vec![
                "First First thing",
                "Something",
                "First thing",
                "Another thing",
                ""
            ]
        );
    }

    #[test]
    fn test_multiple_row_with_empty_column() {
        let mut cm = ColumnMap::new();
        cm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        let mut rows = Vec::new();
        rows.insert(0, vec![]);
        cl! {
            ins cm, rows[0],
            kv "c0", "c0v",
            kv "c1", "Something",
            kv "c2", "v2",
            kv "c3", "32"
        }
        rows.insert(1, vec![]);
        cl! {
            ins cm, rows[1],
            kv "c0", "c0v",
            kv "c2", "v2",
            kv "c3", "32"
        }
        rows.insert(2, vec![]);
        cl! {
            ins cm, rows[2],
            kv "c0", "c0v",
            kv "c1", "Something",
            kv "c2", "v2"
        }
        assert_eq!(
            rows,
            vec![
                vec!["c0v", "Something", "v2", "32"],
                vec!["c0v", "", "v2", "32"],
                vec!["c0v", "Something", "v2", ""],
            ]
        );
    }

    #[test]
    fn test_multi_datasets_csv() {
        let mut cm = ColumnMap::new();
        cm.add_columns(vec!["c0", "c1"]);
        let mut rows = Vec::new();
        // insert data for first dataset
        rows.push(vec![]);
        cl! {
            ins cm, rows[0],
            kv "c0", "c0v",
            kv "c1", "Something"
        }
        cm.add_columns(vec!["c4", "c5"]);
        // insert data for second dataset
        rows.push(vec![]);
        cl! {
            ins cm, rows[1],
            kv "c4", "v2",
            kv "c5", "32"
        }
        // mixture of dataset is possible
        cm.add_columns(vec!["c1", "c5"]);
        rows.push(vec![]);
        cl! {
            ins cm, rows[2],
            kv "c1", "another set",
            kv "c5", "mixed dataset"
        }
        assert_eq!(
            rows,
            vec![
                vec!["c0v", "Something"],
                vec!["", "", "v2", "32"],
                vec!["", "another set", "", "mixed dataset"],
            ]
        );
    }

    #[test]
    fn test_handling_unset_columns() {
        let mut cm = ColumnMap::new();
        cm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        let mut rows = Vec::new();

        rows.push(vec![]);
        cl! {
            ins cm, rows[0],
            kv "c0", "r1d0",
            kv "c2", "r1d2"
        }

        cm.add_column("c4");

        // this will cause a NoDataSet error, cause column c4 was created after setting *this* row
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
}
