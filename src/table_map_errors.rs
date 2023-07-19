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
