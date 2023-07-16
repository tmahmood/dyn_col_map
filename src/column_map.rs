use crate::column_map::colum_map_errors::ColumnMapErrors;
use indexmap::IndexMap;

pub mod colum_map_errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum ColumnMapErrors {
        #[error("Column name does not exist")]
        InvalidColumnName,

        #[error("No data is set")]
        NoDataSet,
    }
}

///
/// Macro to inserting data simpler and clean
/// ```
/// use dyn_col_map::cl;
/// use dyn_col_map::column_map::ColumnMap;
///
/// let mut cm = ColumnMap::new();
/// cm.add_columns(vec!["col_0", "col_1", "col_2", "col_3"]);
/// let mut row = vec![];
/// cl! { ins cm, row, "col_0", "Some value" }
/// cl! {
///     ins cm, row,
///     kv "col_1", "Something",
///     kv "col_2", "another thing",
///     kv "col_3", "more thing"
/// }
/// ```
///
#[macro_export]
macro_rules! cl {
    (ins $cm: ident, $c: expr, $cn: expr, $m: expr) => {
        $cm.insert(&mut $c, $cn, $m).unwrap()
    };

    (ins $cm: ident, $c: expr, $(kv $cn: expr, $m: expr),+) => {{
        $(cl! { ins $cm, $c, $cn, $m });+
    }};
}

#[derive(Clone)]
pub struct ColumnMap {
    columns: IndexMap<String, usize>,
    col_index: usize,
}

impl ColumnMap {
    pub fn new() -> Self {
        Self {
            columns: IndexMap::new(),
            col_index: 0,
        }
    }

    /// Column, in sequence, can be used as headers when generating a CSV file
    pub fn get_columns(&self) -> Vec<String> {
        self.columns.keys().cloned().collect()
    }

    /// Adds a column
    pub fn add_column(&mut self, col_name: &str) {
        if self.columns.contains_key(col_name) {
            return;
        }
        self.columns.insert(col_name.to_string(), self.col_index);
        self.col_index += 1;
    }

    /// Adds multiple columns, the sequence will be maintained.
    pub fn add_columns(&mut self, cols: Vec<&str>) {
        for col in cols {
            self.add_column(col)
        }
    }

    fn fill_target<T: Default>(target: &mut Vec<T>, end: &usize, start: usize) {
        for ii in start..=*end {
            if let None = target.get(ii) {
                target.push(T::default())
            }
        }
    }

    /// Inserts value in the target vec in the given index. If there's not enough elements,
    /// it will fill it up with default value, and then insert the value in required position
    pub fn insert<T: Default>(
        &self,
        target: &mut Vec<T>,
        col_name: &str,
        value: T,
    ) -> Result<(), ColumnMapErrors> {
        let index = match self.columns.get(col_name) {
            None => return Err(ColumnMapErrors::InvalidColumnName),
            Some(index) => index,
        };
        self.fill_to_end(target);
        target[*index] = value;
        Ok(())
    }

    /// If there are more columns than the target row, fills, all the missing columns
    /// with default value
    pub fn fill_to_end<T: Default>(&self, target: &mut Vec<T>) {
        let n = self.columns.len() - 1;
        Self::fill_target(target, &n, 0);
    }

    /// gets data from the given array, using the column name.
    pub fn get<T: Clone>(&self, target: &Vec<T>, col_name: &str) -> Result<T, ColumnMapErrors> {
        let index = match self.columns.get(col_name) {
            None => return Err(ColumnMapErrors::InvalidColumnName),
            Some(index) => index,
        };
        match target.get(*index) {
            None => return Err(ColumnMapErrors::NoDataSet),
            Some(v) => Ok(v.clone()),
        }
    }
}
