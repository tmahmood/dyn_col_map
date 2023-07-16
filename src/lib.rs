pub mod table_map;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table_map::table_map_errors::TableMapErrors;
    use crate::table_map::TableMap;

    #[test]
    fn test_macro() {
        let mut tm = TableMap::new();
        tm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        update_row! { tm, "c0", "c0v" }
        update_row! {
            tm,
            "c1", "Something",
            "c2", "v2",
            "c3", "32"
        }
        // get all the columns, sequence is maintained
        assert_eq!(tm.get_columns(), vec!["c0", "c1", "c2", "c3"]);
        assert_eq!(tm.get_vec(), &vec![vec!["c0v", "Something", "v2", "32"]]);
    }

    #[test]
    fn test_macro_obj() {
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
        let mut tm = TableMap::new();
        tm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        update_row! { tm, "c0", ar[0].clone() }
        update_row! {
            tm,
            "c1", ar[1].clone(),
            "c2", ar[2].clone(),
            "c3", ar[3].clone()
        }
        assert_eq!(tm.get_vec(), &vec![ar])
    }

    #[test]
    fn test_insert_randomly() {
        let mut tm = TableMap::new();
        tm.add_columns(vec!["c0", "c1", "c2", "c3"]);

        update_row! {
            tm,
            "c1", "Something",
            "c3", "Another thing",
            "c2", "First thing"
        }
        assert_eq!(tm.get_current("c1").unwrap(), "Something");
        assert!(tm.get_current("c10").is_err());
        assert_eq!(
            tm.get_vec(),
            &vec![vec!["", "Something", "First thing", "Another thing"]]
        );
    }

    #[test]
    fn test_extending_with_new_column() {
        let mut tm = TableMap::new();
        tm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        update_row! {
            tm,
            "c1", "Something",
            "c3", "Another thing",
            "c2", "First thing"
        }
        tm.add_column("c5");
        tm.insert("c0", "First First thing").unwrap();
        // no matter how the data is inserted, the sequence of column is maintained
        assert_eq!(
            tm.get_vec(),
            &vec![vec![
                "First First thing",
                "Something",
                "First thing",
                "Another thing",
                "",
            ]]
        );
    }

    #[test]
    fn test_multiple_row_with_empty_column() {
        let mut tm = TableMap::new();
        tm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        push! {
            tm,
            "c0", "c0v",
            "c1", "Something",
            "c2", "v2",
            "c3", "32"
        }
        push! {
            tm,
            "c0", "c0v",
            "c2", "v2",
            "c3", "32"
        }
        push! {
            tm,
            "c0", "c0v",
            "c1", "Something",
            "c2", "v2"
        }
        assert_eq!(
            tm.get_vec(),
            &vec![
                vec!["c0v", "Something", "v2", "32"],
                vec!["c0v", "", "v2", "32"],
                vec!["c0v", "Something", "v2", ""],
            ]
        );
    }

    #[test]
    fn test_multi_datasets_csv() {
        let mut tm = TableMap::new();
        tm.add_columns(vec!["c0", "c1"]);
        // insert data for first dataset
        push! {
            tm,
            "c0", "c0v",
            "c1", "Something"
        }
        tm.add_columns(vec!["c4", "c5"]);
        // insert data for second dataset
        push! {
            tm,
            "c4", "v2",
            "c5", "32"
        }
        // mixture of dataset is possible
        tm.add_columns(vec!["c1", "c5"]);
        push! {
            tm,
            "c1", "another set",
            "c5", "mixed dataset"
        }
        assert_eq!(
            tm.get_vec(),
            &vec![
                vec!["c0v", "Something", "", ""],
                vec!["", "", "v2", "32"],
                vec!["", "another set", "", "mixed dataset"],
            ]
        );
    }

    // testing unset columns
    fn setup_for_unset_columns() -> TableMap<String> {
        let mut tm = TableMap::new();
        tm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        update_row! {
            tm,
            "c0", "r1d0".into(),
            "c2", "r1d2".into()
        }
        tm
    }

    #[test]
    fn test_unset_column_value_should_be_empty() {
        let mut tm = setup_for_unset_columns();
        // this will be an empty value, as inserted row does not set "c3" column
        assert_eq!(tm.get_current("c3").unwrap(), "");
    }

    #[test]
    fn test_accessing_rows_added_before_additional_column_returns_error() {
        let mut tm = setup_for_unset_columns();
        tm.add_column("c4");
        // this will cause a NoDataSet error, cause column c4 was created after setting *this* row
        assert!(tm.get_current("c4").is_err());
    }

    #[test]
    fn test_filling_unset_columns() {
        let mut tm = setup_for_unset_columns();
        tm.add_column("c4");
        tm.fill_to_end();
        assert!(tm.get_current("c4").is_ok());
    }

    #[test]
    fn test_before_moving_to_next_row_will_fill_up_current_row() {
        let mut tm = setup_for_unset_columns();
        tm.add_column("c4");
        tm.next_row();
        assert!(tm.get_index(0, "c4").is_ok());
    }
}
