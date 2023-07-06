use crate::column_mapper::col_mapper_errors::ColMapperErrors;
use indexmap::IndexMap;

pub mod col_mapper_errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum ColMapperErrors {
        #[error("Column name does not exist")]
        InvalidColumnName,

        #[error("No data is set")]
        NoDataSet,
    }
}

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
pub struct ColumnMapper {
    columns: IndexMap<String, usize>,
    col_index: usize,
}

impl ColumnMapper {
    pub fn new() -> Self {
        Self {
            columns: IndexMap::new(),
            col_index: 0,
        }
    }

    pub fn get_columns(&self) -> Vec<String> {
        self.columns.keys().cloned().collect()
    }

    pub fn add_column(&mut self, col_name: &str) {
        if self.columns.contains_key(col_name) {
            return;
        }
        self.columns.insert(col_name.to_string(), self.col_index);
        self.col_index += 1;
    }

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

    pub fn insert<T: Default>(
        &self,
        target: &mut Vec<T>,
        col_name: &str,
        value: T,
    ) -> Result<(), ColMapperErrors> {
        let index = match self.columns.get(col_name) {
            None => return Err(ColMapperErrors::InvalidColumnName),
            Some(index) => index,
        };
        Self::fill_target(target, index, 0);
        target[*index] = value;
        self.fill_to_end(target);
        Ok(())
    }

    pub fn fill_to_end<T: Default>(&self, target: &mut Vec<T>) {
        let n = self.columns.len() - 1;
        Self::fill_target(target, &n, 0);
    }

    pub fn get<T: Clone>(&self, target: &Vec<T>, col_name: &str) -> Result<T, ColMapperErrors> {
        let index = match self.columns.get(col_name) {
            None => return Err(ColMapperErrors::InvalidColumnName),
            Some(index) => index,
        };
        match target.get(*index) {
            None => return Err(ColMapperErrors::NoDataSet),
            Some(v) => Ok(v.clone()),
        }
    }
}
