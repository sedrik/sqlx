use crate::arguments::Arguments;
use crate::connection::Connection;
use crate::row::Row;
use crate::types::HasTypeMetadata;

/// A database driver.
///
/// This trait encapsulates a complete driver implementation to a specific
/// database (e.g., MySQL, Postgres).
pub trait Database: HasTypeMetadata + 'static {
    /// The concrete `Connection` implementation for this database.
    type Connection: Connection<Database = Self>;

    /// The concrete `Arguments` implementation for this database.
    type Arguments: Arguments<Database = Self>;

    /// The concrete `Row` implementation for this database.
    type Row: Row<Database = Self>;
}