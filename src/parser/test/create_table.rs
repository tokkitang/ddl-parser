#![cfg(test)]

use crate::ast::ddl::CreateTableQuery;
use crate::ast::types::{Column, DataType, TableName};
use crate::parser::context::ParserContext;
use crate::parser::predule::Parser;

#[test]
pub fn create_table() {
    let text = r#"
        CREATE TABLE "test_db".person
        (
            id INTEGER PRIMARY KEY,
            name varchar(100),
            age INTEGER
        );
    "#
    .to_owned();

    let mut parser = Parser::new(text).unwrap();

    let expected = CreateTableQuery::builder()
        .set_table(TableName::new(
            Some("test_db".to_owned()),
            "person".to_owned(),
        ))
        .add_column(
            Column::builder()
                .set_name("id".to_owned())
                .set_data_type(DataType::Int)
                .set_primary_key(true)
                .build(),
        )
        .add_column(
            Column::builder()
                .set_name("name".to_owned())
                .set_data_type(DataType::Varchar(100))
                .build(),
        )
        .add_column(
            Column::builder()
                .set_name("age".to_owned())
                .set_data_type(DataType::Int)
                .build(),
        )
        .build();

    assert_eq!(
        parser.parse(ParserContext::default()).unwrap(),
        vec![expected],
    );
}

#[test]
pub fn create_table_2() {
    let text = r#"
        CREATE TABLE "test_db".person
        (
            id INTEGER PRIMARY KEY,
            name varchar(100) NOT NULL,
            age INTEGER NULL
        );
    "#
    .to_owned();

    let mut parser = Parser::new(text).unwrap();

    let expected = CreateTableQuery::builder()
        .set_table(TableName::new(
            Some("test_db".to_owned()),
            "person".to_owned(),
        ))
        .add_column(
            Column::builder()
                .set_name("id".to_owned())
                .set_data_type(DataType::Int)
                .set_primary_key(true)
                .build(),
        )
        .add_column(
            Column::builder()
                .set_name("name".to_owned())
                .set_data_type(DataType::Varchar(100))
                .set_not_null(true)
                .build(),
        )
        .add_column(
            Column::builder()
                .set_name("age".to_owned())
                .set_data_type(DataType::Int)
                .build(),
        )
        .build();

    assert_eq!(
        parser.parse(ParserContext::default()).unwrap(),
        vec![expected],
    );
}

#[test]
pub fn create_table_3() {
    let text = r#"
        CREATE TABLE "test_db".person
        (
            id INTEGER PRIMARY KEY DEFAULT 0,
            name varchar(100) NOT NULL DEFAULT 'test',
            age INTEGER NULL DEFAULT 0
        );
    "#
    .to_owned();

    let mut parser = Parser::new(text).unwrap();

    let expected = CreateTableQuery::builder()
        .set_table(TableName::new(
            Some("test_db".to_owned()),
            "person".to_owned(),
        ))
        .add_column(
            Column::builder()
                .set_name("id".to_owned())
                .set_data_type(DataType::Int)
                .set_primary_key(true)
                .set_default("0".to_owned())
                .build(),
        )
        .add_column(
            Column::builder()
                .set_name("name".to_owned())
                .set_data_type(DataType::Varchar(100))
                .set_not_null(true)
                .set_default("test".to_owned())
                .build(),
        )
        .add_column(
            Column::builder()
                .set_name("age".to_owned())
                .set_data_type(DataType::Int)
                .set_default("0".to_owned())
                .build(),
        )
        .build();

    assert_eq!(
        parser.parse(ParserContext::default()).unwrap(),
        vec![expected],
    );
}
