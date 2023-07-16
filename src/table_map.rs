use crate::table_map::table_map_errors::TableMapErrors;
use indexmap::IndexMap;
use std::fmt::Debug;
use std::mem;
use std::ops::Deref;
use std::rc::Rc;

pub mod table_map_errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum TableMapErrors {
        #[error("Column name does not exist")]
        InvalidColumnName,

        #[error("No data is set")]
        NoDataSet,

        #[error("Invalid row index")]
        InvalidRowIndex,
    }
}

///
/// Updates the current row, it will create a new row if no rows exists
/// ```
/// use dyn_col_map::update_row;
/// use dyn_col_map::table_map::TableMap;
///
/// let mut cm = TableMap::new();
/// cm.add_columns(vec!["col_0", "col_1", "col_2", "col_3"]);
/// let mut row = vec![];
/// update_row! { cm, "col_0", "Some value" }
/// update_row! {
///     cm,
///     "col_1", "Something",
///     "col_2", "another thing",
///     "col_3", "more thing"
/// }
/// ```
///
#[macro_export]
macro_rules! update_row {

    ($cm: ident, $cn: expr, $m: expr) => {
        $cm.insert($cn, $m).unwrap()
    };

    ($cm: ident, $($cn: expr, $m: expr),+) => {{
        $(update_row! { $cm, $cn, $m });+
    }};

}

///
/// insert data in a new row
/// ```
/// use dyn_col_map::push;
/// use dyn_col_map::table_map::TableMap;
///
/// let mut cm = TableMap::new();
/// cm.add_columns(vec!["col_0", "col_1", "col_2", "col_3"]);
/// push! { cm, "col_0", "Some value" }
/// push! {
///     cm,
///     "col_1", "Something",
///     "col_2", "another thing",
///     "col_3", "more thing"
/// }
/// ```
///
#[macro_export]
macro_rules! push {
    ($cm: ident, $cn: expr, $m: expr) => {
        $cm.next_row();
        $cm.insert($cn, $m).unwrap();
    };

    ($cm: ident, $($cn: expr, $m: expr),+) => {{
        $cm.next_row();
        $(update_row! { $cm, $cn, $m });+
    }};
}

#[derive(Clone)]
pub struct TableMap<T: Default + Clone + Debug> {
    columns: IndexMap<String, usize>,
    col_index: usize,
    rows: Vec<Vec<T>>,
}

impl<T: Default + Clone + Debug> TableMap<T> {
    pub fn new() -> Self {
        Self {
            columns: IndexMap::new(),
            col_index: 0,
            rows: vec![],
        }
    }

    /// Column, in sequence, can be used as headers when generating a CSV file
    pub fn get_columns(&self) -> Vec<String> {
        self.columns.keys().cloned().collect()
    }

    /// insert current row to main collection, clears the current row
    pub fn next_row(&mut self) {
        if self.rows.len() > 0 {
            self.fill_to_end();
        }
        self.rows.push(vec![]);
        self.fill_to_end();
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

    fn get_current_row(&self) -> &Vec<T> {
        self.rows.last().unwrap()
    }

    fn get_current_row_mut(&mut self) -> &mut Vec<T> {
        if self.rows.last().is_none() {
            self.rows.push(vec![])
        }
        self.rows.last_mut().unwrap()
    }

    fn fill_target(&mut self, end: &usize, start: usize) {
        let mut current_row = self.get_current_row_mut();
        for ii in start..=*end {
            if let None = current_row.get(ii) {
                current_row.push(T::default())
            }
        }
    }

    /// Inserts value in the target vec in the given index. If there's not enough elements,
    /// it will fill it up with default value, and then insert the value in required position
    pub fn insert(&mut self, col_name: &str, value: T) -> Result<(), TableMapErrors> {
        let index = self.get_column_index(col_name)?;
        self.fill_to_end();
        let current_row = self.get_current_row_mut();
        current_row[index] = value;
        Ok(())
    }

    fn get_column_index(&self, col_name: &str) -> Result<usize, TableMapErrors> {
        self.columns
            .get(col_name)
            .ok_or(TableMapErrors::InvalidColumnName)
            .cloned()
    }

    /// If there are more columns than the target row, fills, all the missing columns
    /// with default value
    pub fn fill_to_end(&mut self) {
        let n = self.columns.len() - 1;
        self.fill_target(&n, 0);
    }

    /// gets data from current row, using the column name.
    pub fn get_current(&self, col_name: &str) -> Result<T, TableMapErrors> {
        let index = self.get_column_index(col_name)?;
        let current_row = self.get_current_row();
        current_row
            .get(index)
            .ok_or(TableMapErrors::NoDataSet)
            .cloned()
    }

    /// gets data from indexd row, using the column name.
    pub fn get_index(&self, row_index: usize, col_name: &str) -> Result<T, TableMapErrors> {
        let index = self.get_column_index(col_name)?;
        let selected_row = self
            .rows
            .get(row_index)
            .ok_or(TableMapErrors::InvalidRowIndex)?;
        selected_row
            .get(index)
            .ok_or(TableMapErrors::NoDataSet)
            .cloned()
    }

    pub fn get_vec(&self) -> &Vec<Vec<T>> {
        &self.rows
    }
}
