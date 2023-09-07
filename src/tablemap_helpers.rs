#[macro_export]
macro_rules! setters_fn {
    () => {

        impl From<Columns> for String {
            fn from(value: Columns) -> Self {
                let string: &str = value.into();
                string.to_string()
            }
        }
        pub fn columns(tm: &mut TableMap<String>) {
            tm.add_columns(Columns::iter().map(|v| v.into()).collect());
        }

        pub fn upd_str(tm: &mut TableMap<String>, row: usize, col: Columns, val: &str) {
            tm.update_row(row, col.into(), val.to_string()).unwrap();
        }

        pub fn upd_string(tm: &mut TableMap<String>, row: usize, col: Columns, val: &str) {
            tm.update_row(row, col.into(), val.to_string()).unwrap();
        }

        pub fn ins_str(tm: &mut TableMap<String>, c: Columns, v: &str) {
            tm.insert(c.into(), v.to_string()).unwrap();
        }

        pub fn ins_string(tm: &mut TableMap<String>, c: Columns, v: String) {
            tm.insert(c.into(), v).unwrap();
        }

        pub fn get_column(
            tm: &TableMap<String>,
            c: Columns,
            index: Option<usize>,
        ) -> Result<String, TableMapErrors> {
            let cname: String = c.into();
            if let Some(idx) = index {
                tm.get_column_value_by_index(idx, &cname)
            } else {
                tm.get_column_value(&cname)
            }
        }
    };
}

#[macro_export]
macro_rules! col {
    ($tm: expr) => {
        $tm.add_columns(Columns::iter().map(|v| v.into()).collect());
    };
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;
    use strum_macros::{EnumIter, EnumString, IntoStaticStr};

    use crate::table_map_errors::TableMapErrors;
    use crate::TableMap;

    #[derive(EnumIter, IntoStaticStr, EnumString)]
    pub enum Columns {
        Name,
        Address,
    }

    setters_fn!();

    #[test]
    fn test_adding_column_from_enum() {
        let mut tm: TableMap<String> = TableMap::new();
        col!(tm);
        assert_eq!(
            tm.get_columns(),
            vec!["Name".to_string(), "Address".to_string()]
        );

        ins_str(&mut tm, Columns::Name, "John");
        ins_str(&mut tm, Columns::Address, "Unknown");
        tm.next_row();
        ins_str(&mut tm, Columns::Name, "Doe");
        ins_str(&mut tm, Columns::Name, "Still not known");
        upd_str(&mut tm, 0, Columns::Address, "Searching")
    }
}