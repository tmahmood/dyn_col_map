pub mod column_mapper;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::column_mapper::col_mapper_errors::ColMapperErrors;
    use crate::column_mapper::ColumnMapper;

    #[test]
    fn test_col_mapper_macro() {
        let mut cm = ColumnMapper::new();
        cm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        let mut p = vec![];
        cl! { ins cm, p, "c0", "c0v" }
        cl! {
            ins cm, p,
            kv "c1", "Something",
            kv "c2", "v2",
            kv "c3", "32"
        }
        assert_eq!(cm.get_columns(), vec!["c0", "c1", "c2", "c3"]);
        assert_eq!(p, vec!["c0v", "Something", "v2", "32"])
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
        let mut column_mapper = ColumnMapper::new();
        column_mapper.add_columns(vec!["c0", "c1", "c2", "c3"]);
        let mut p = vec![];
        cl! { ins column_mapper, p, "c0", ar[0].clone() }
        cl! {
            ins column_mapper, p,
            kv "c1", ar[1].clone(),
            kv "c2", ar[2].clone(),
            kv "c3", ar[3].clone()
        }
        assert_eq!(column_mapper.get_columns(), vec!["c0", "c1", "c2", "c3"]);
        assert_eq!(p, ar)
    }

    #[test]
    fn test_insert_randomly() {
        let mut cm = ColumnMapper::new();
        cm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        let mut p = Vec::new();

        cm.insert(&mut p, "c1", "Something").unwrap();
        cm.insert(&mut p, "c3", "Another thing").unwrap();
        cm.insert(&mut p, "c2", "First thing").unwrap();

        assert_eq!(cm.get(&p, "c1").unwrap(), "Something");

        assert!(cm.get(&p, "c10").is_err());

        assert_eq!(p, vec!["", "Something", "First thing", "Another thing"]);
    }

    #[test]
    fn test_extending_with_new_column() {
        let mut cm = ColumnMapper::new();
        cm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        let mut p = Vec::new();

        cm.insert(&mut p, "c1", "Something").unwrap();
        cm.insert(&mut p, "c3", "Another thing").unwrap();
        cm.insert(&mut p, "c2", "First thing").unwrap();

        cm.add_column("c5");
        cm.insert(&mut p, "c0", "First First thing").unwrap();

        assert_eq!(
            p,
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
        let mut cm = ColumnMapper::new();
        cm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        let mut p = Vec::new();
        p.insert(0, vec![]);
        cl! {
            ins cm, p[0],
            kv "c0", "c0v",
            kv "c1", "Something",
            kv "c2", "v2",
            kv "c3", "32"
        }
        p.insert(1, vec![]);
        cl! {
            ins cm, p[1],
            kv "c0", "c0v",
            kv "c2", "v2",
            kv "c3", "32"
        }
        p.insert(2, vec![]);
        cl! {
            ins cm, p[2],
            kv "c0", "c0v",
            kv "c1", "Something",
            kv "c2", "v2"
        }
        assert_eq!(
            p,
            vec![
                vec!["c0v", "Something", "v2", "32"],
                vec!["c0v", "", "v2", "32"],
                vec!["c0v", "Something", "v2", ""],
            ]
        );
    }
}
