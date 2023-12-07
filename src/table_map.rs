use std::fmt::Debug;
use std::slice::Iter;

use indexmap::IndexMap;

use crate::table_map_errors::TableMapErrors;

///
/// Updates the current row, it will create a new row if no rows exists
/// ```
/// use table_map::{update_row, TableMap};
///
/// let mut cm = TableMap::new();
/// cm.add_columns(vec!["col_0", "col_1", "col_2", "col_3"]);
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
        $($cm.insert($cn, $m).unwrap();)+
    }};

}

///
/// insert data in a new row
/// ```
/// use table_map::{push, update_row, TableMap};
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
        $($cm.insert($cn, $m).unwrap();)+
    }};
}

#[derive(Clone)]
pub struct TableMap<T: Default + Debug + Clone> {
    columns: IndexMap<String, usize>,
    col_index: usize,
    rows: Vec<Vec<T>>,
    is_current_row_dirty: bool
}

impl<T: Default + Debug + Clone> TableMap<T> {
    pub fn new() -> Self {
        Self {
            columns: IndexMap::new(),
            col_index: 0,
            rows: vec![],
            is_current_row_dirty: true
        }
    }

    /// returns current row index, None if there is no rows inserted yet
    pub fn current_row_index(&self) -> Option<usize> {
        if self.rows.len() == 0 { return None }
        Some(self.rows.len() - 1)
    }
    /// number of rows
    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }

    /// number of columns
    pub fn num_cols(&self) -> usize {
        self.columns.len()
    }

    /// Column, in sequence, can be used as headers when generating a CSV file
    pub fn get_columns(&self) -> Vec<String> {
        self.columns.keys().cloned().collect()
    }

    /// moves to next row.
    pub fn next_row(&mut self) {
        self.rows.push(vec![T::default(); self.columns.len()]);
    }

    /// copies current row, and push it at the end, creating duplicate
    pub fn copy_row(&mut self) {
        let new_row = self.rows.last().cloned().unwrap();
        self.rows.push(new_row);
    }

    /// copies the row given by row_index, and push it at the end, creating duplicate
    pub fn copy_row_at_index(&mut self, row_index: usize) {
        let new_row = self.rows.get( row_index).cloned().unwrap();
        self.rows.push(new_row);
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

    /// Inserts value in the target vec in the given index. If there's not enough elements,
    /// it will fill it up with default value, and then insert the value in required position
    pub fn insert(&mut self, col_name: &str, value: T) -> Result<(), TableMapErrors> {
        let index = self.get_column_index(col_name)?;
        self.fill_to_end();
        let current_row = self.get_current_row_mut();
        current_row[index] = value;
        Ok(())
    }

    pub fn get_column_index(&self, col_name: &str) -> Result<usize, TableMapErrors> {
        self.columns
            .get(col_name)
            .ok_or(TableMapErrors::InvalidColumnName)
            .cloned()
    }

    /// If there are more columns than the target row, fills, all the missing columns
    /// with default value
    pub fn fill_to_end(&mut self) {
        let n = self.col_index - 1;
        self.fill_target(&n, 0);
    }

    fn fill_target(&mut self, end: &usize, start: usize) {
        let current_row = self.get_current_row_mut();
        for ii in start..=*end {
            if let None = current_row.get(ii) {
                current_row.push(T::default())
            }
        }
    }

    /// gets data from current row, using the column name.
    pub fn get_column_value(&self, col_name: &str) -> Result<T, TableMapErrors> {
        let index = self.get_column_index(col_name)?;
        let current_row = self.get_current_row()?;
        current_row
            .get(index)
            .ok_or(TableMapErrors::NoDataSet)
            .cloned()
    }

    /// gets data from indexed row, using the column name.
    pub fn get_column_value_by_index(
        &self,
        row_index: usize,
        col_name: &str,
    ) -> Result<T, TableMapErrors> {
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

    /// get all the data, this returns reference, so will not take any additional memory
    pub fn get_vec(&self) -> &Vec<Vec<T>> {
        &self.rows
    }

    pub fn update_row(&mut self, row_index: usize, col_name: &str, new_val: T) -> Result<(), TableMapErrors> {
        let index = self.get_column_index(col_name)?;
        let row = self.get_row_by_index_mut(row_index)?;
        if let Some(val) = row.get_mut(index) {
            *val = new_val;
        }
        Ok(())
    }

    fn get_row_by_index_mut(&mut self, row: usize) -> Result<&mut Vec<T>, TableMapErrors> {
        self.rows
            .get_mut(row)
            .ok_or(TableMapErrors::InvalidRowIndex)
    }

    pub fn get_current_row(&self) -> Result<&Vec<T>, TableMapErrors> {
        self.rows.last().ok_or(TableMapErrors::NoDataSet)
    }

    fn get_current_row_mut(&mut self) -> &mut Vec<T> {
        if self.rows.last().is_none() {
            self.rows.push(vec![T::default(); self.columns.len()])
        }
        self.rows.last_mut().unwrap()
    }

    /// returns iter for the inner vec
    pub fn iter(&self) -> Iter<'_, Vec<T>> {
        self.rows.iter()
    }

}
