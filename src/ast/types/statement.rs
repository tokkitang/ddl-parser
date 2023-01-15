use crate::ast::ddl::{
    AlterDatabaseQuery, AlterTableQuery, CreateDatabaseQuery, CreateIndexQuery, CreateTableQuery,
    DropDatabaseQuery, DropTableQuery,
};

#[derive(Clone, Debug, PartialEq)]
pub enum SQLStatement {
    DDL(DDLStatement),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DDLStatement {
    CreateDatabaseQuery(CreateDatabaseQuery),
    AlterDatabase(AlterDatabaseQuery),
    DropDatabaseQuery(DropDatabaseQuery),
    CreateTableQuery(CreateTableQuery),
    AlterTableQuery(AlterTableQuery),
    DropTableQuery(DropTableQuery),
    CreateIndexQuery(CreateIndexQuery),
}
