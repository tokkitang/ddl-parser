use std::error::Error;

use crate::ast::predule::{Column, DataType, TableName};
use crate::errors::predule::ParsingError;
use crate::lexer::predule::Token;
use crate::parser::predule::{Parser, ParserContext};

impl Parser {
    // 테이블 컬럼 정의 분석
    pub(crate) fn parse_table_column(&mut self) -> Result<Column, Box<dyn Error + Send>> {
        let mut builder = Column::builder();

        if !self.has_next_token() {
            return Err(ParsingError::boxed("E0001 need more tokens"));
        }

        let current_token = self.get_next_token();

        if let Token::Identifier(name) = current_token {
            builder = builder.set_name(name);
        } else {
            return Err(ParsingError::boxed(format!(
                "E0028 expected identifier. but your input word is '{:?}'",
                current_token
            )));
        }

        let data_type = self.parse_data_type()?;
        builder = builder.set_data_type(data_type);

        loop {
            if !self.has_next_token() {
                break;
            }

            let current_token = self.get_next_token();

            match current_token {
                Token::Comma => {
                    // , 만나면 종료
                    break;
                }
                Token::RightParentheses => {
                    // ) 만나면 종료
                    self.unget_next_token(current_token);
                    break;
                }
                Token::Primary => {
                    if !self.has_next_token() {
                        return Err(ParsingError::boxed("E0003 need more tokens"));
                    }

                    let current_token = self.get_next_token();

                    match current_token {
                        Token::Key => {
                            builder = builder.set_primary_key(true).set_not_null(true);
                        }
                        _ => {
                            return Err(ParsingError::boxed(format!(
                                "expected 'PRIMARY KEY'. but your input word is '{:?}'",
                                current_token
                            )));
                        }
                    }
                }
                Token::Not => {
                    if !self.has_next_token() {
                        return Err(ParsingError::boxed("E0004 need more tokens"));
                    }

                    let current_token = self.get_next_token();

                    match current_token {
                        Token::Null => {
                            builder = builder.set_not_null(true);
                        }
                        _ => {
                            return Err(ParsingError::boxed(format!(
                                "expected 'NOT NULL'. but your input word is '{:?}'",
                                current_token
                            )));
                        }
                    }
                }
                Token::Null => {
                    builder = builder.set_not_null(false);
                }
                Token::Comment => {
                    if !self.has_next_token() {
                        return Err(ParsingError::boxed("E0005 need more tokens"));
                    }

                    let current_token = self.get_next_token();

                    if let Token::String(comment) = current_token {
                        builder = builder.set_comment(comment);
                    } else {
                        return Err(ParsingError::boxed(format!(
                            "expected comment string. but your input word is '{:?}'",
                            current_token
                        )));
                    }
                }
                Token::Default => {
                    return Err(ParsingError::boxed("not supported yet"));
                }
                _ => {}
            }
        }

        Ok(builder.build())
    }

    // 데이터 타입 분석
    pub(crate) fn parse_data_type(&mut self) -> Result<DataType, Box<dyn Error + Send>> {
        if !self.has_next_token() {
            return Err(ParsingError::boxed("E0006 need more tokens"));
        }

        let current_token = self.get_next_token();

        if let Token::Identifier(type_name) = current_token {
            match type_name.to_uppercase().as_str() {
                "INTEGER" | "INT" => Ok(DataType::Int),
                "FLOAT" => Ok(DataType::Float),
                "BOOLEAN" | "BOOL" => Ok(DataType::Boolean),
                "VARCHAR" => {
                    // 여는 괄호 체크
                    if !self.has_next_token() {
                        return Err(ParsingError::boxed("E0007 need more tokens"));
                    }

                    let current_token = self.get_next_token();

                    if Token::LeftParentheses != current_token {
                        return Err(ParsingError::boxed(format!(
                            "expected '('. but your input word is '{:?}'",
                            current_token
                        )));
                    }

                    // 문자열 길이 체크
                    if !self.has_next_token() {
                        return Err(ParsingError::boxed("E0008 need more tokens"));
                    }

                    let current_token = self.get_next_token();

                    if let Token::Integer(integer) = current_token {
                        // 닫는 괄호 체크
                        if !self.has_next_token() {
                            return Err(ParsingError::boxed("E0009 need more tokens"));
                        }

                        let current_token = self.get_next_token();

                        if Token::RightParentheses != current_token {
                            return Err(ParsingError::boxed(format!(
                                "expected ')'. but your input word is '{:?}'",
                                current_token
                            )));
                        }

                        Ok(DataType::Varchar(integer))
                    } else {
                        Err(ParsingError::boxed(format!(
                            "expected integer number. but your input word is '{:?}'",
                            current_token
                        )))
                    }
                }
                _ => Err(ParsingError::boxed(format!(
                    "unknown data type '{}'",
                    type_name
                ))),
            }
        } else {
            Err(ParsingError::boxed(format!(
                "E0029 expected identifier. but your input word is '{:?}'",
                current_token
            )))
        }
    }

    // 테이블명 분석
    pub(crate) fn parse_table_name(
        &mut self,
        context: ParserContext,
    ) -> Result<TableName, Box<dyn Error + Send>> {
        // 테이블명 획득 로직
        if !self.has_next_token() {
            return Err(ParsingError::boxed("E0010 need more tokens"));
        }

        // 첫번째로 오는 이름은 테이블명으로 추정
        let current_token = self.get_next_token();
        let mut database_name = None;

        let mut table_name = if let Token::Identifier(name) = current_token {
            name
        } else {
            return Err(ParsingError::boxed(format!(
                "E0030 expected identifier. but your input word is '{:?}'",
                current_token
            )));
        };

        if !self.has_next_token() {
            return Ok(TableName::new(
                database_name.or(context.default_database),
                table_name,
            ));
        }

        let current_token = self.get_next_token();

        // .가 있을 경우 "데이터베이스명"."테이블명"의 형태로 추정
        if current_token == Token::Period {
            if !self.has_next_token() {
                return Err(ParsingError::boxed("E0012 need more tokens"));
            }

            let current_token = self.get_next_token();

            if let Token::Identifier(name) = current_token {
                database_name = Some(table_name);
                table_name = name;
            } else {
                return Err(ParsingError::boxed(format!(
                    "E0031 expected identifier. but your input word is '{:?}'",
                    current_token
                )));
            }
        } else {
            self.unget_next_token(current_token);
        }

        Ok(TableName::new(
            database_name.or(context.default_database),
            table_name,
        ))
    }

    // IF NOT EXISTS 체크 로직
    pub(crate) fn has_if_not_exists(&mut self) -> Result<bool, Box<dyn Error + Send>> {
        // 테이블명 획득 로직
        if !self.has_next_token() {
            return Err(ParsingError::boxed("E0013 need more tokens"));
        }

        let current_token = self.get_next_token();

        if Token::If == current_token {
            if !self.has_next_token() {
                return Err(ParsingError::boxed("E0014 need more tokens"));
            }

            let current_token = self.get_next_token();

            if Token::Not == current_token {
                if !self.has_next_token() {
                    return Err(ParsingError::boxed("E0015 need more tokens"));
                }

                let current_token = self.get_next_token();

                if Token::Exists == current_token {
                    Ok(true)
                } else {
                    Err(ParsingError::boxed(format!(
                        "expected keyword is 'exists'. but your input word is '{:?}'",
                        current_token
                    )))
                }
            } else {
                Err(ParsingError::boxed(format!(
                    "expected keyword is 'not'. but your input word is '{:?}'",
                    current_token
                )))
            }
        } else {
            self.unget_next_token(current_token);
            Ok(false)
        }
    }

    // IF EXISTS 체크 로직
    pub(crate) fn has_if_exists(&mut self) -> Result<bool, Box<dyn Error + Send>> {
        // 테이블명 획득 로직
        if !self.has_next_token() {
            return Err(ParsingError::boxed("E0016 need more tokens"));
        }

        let current_token = self.get_next_token();

        if Token::If == current_token {
            if !self.has_next_token() {
                return Err(ParsingError::boxed("E0017 need more tokens"));
            }

            let current_token = self.get_next_token();

            if Token::Exists == current_token {
                Ok(true)
            } else {
                Err(ParsingError::boxed(format!(
                    "expected keyword is 'exists'. but your input word is '{:?}'",
                    current_token
                )))
            }
        } else {
            self.unget_next_token(current_token);
            Ok(false)
        }
    }

    // 다음 토큰이 여는 괄호인지
    pub(crate) fn _next_token_is_subquery(&mut self) -> bool {
        if !self.has_next_token() {
            false
        } else {
            let current_token = self.get_next_token();

            if current_token == Token::LeftParentheses {
                if !self.has_next_token() {
                    self.unget_next_token(current_token);
                    false
                } else {
                    let second_token = self.get_next_token();

                    if second_token == Token::Select {
                        self.unget_next_token(second_token);
                        self.unget_next_token(current_token);
                        true
                    } else {
                        self.unget_next_token(second_token);
                        self.unget_next_token(current_token);
                        false
                    }
                }
            } else {
                self.unget_next_token(current_token);
                false
            }
        }
    }

    // 다음 토큰이 COLUMN인지
    pub(crate) fn next_token_is_column(&mut self) -> bool {
        if !self.has_next_token() {
            false
        } else {
            let current_token = self.get_next_token();

            match current_token {
                Token::Column => {
                    self.unget_next_token(current_token);
                    true
                }
                _ => {
                    self.unget_next_token(current_token);
                    false
                }
            }
        }
    }

    // 다음 토큰이 COLUMN인지
    pub(crate) fn next_token_is_not_null(&mut self) -> bool {
        if !self.has_next_token() {
            false
        } else {
            let first_token = self.get_next_token();

            match first_token {
                Token::Not => {
                    if !self.has_next_token() {
                        self.unget_next_token(first_token);
                        false
                    } else {
                        let second_token = self.get_next_token();

                        match second_token {
                            Token::Null => {
                                self.unget_next_token(second_token);
                                self.unget_next_token(first_token);
                                true
                            }
                            _ => {
                                self.unget_next_token(second_token);
                                self.unget_next_token(first_token);
                                false
                            }
                        }
                    }
                }
                _ => {
                    self.unget_next_token(first_token);
                    false
                }
            }
        }
    }

    // 다음 토큰이 COLUMN인지
    pub(crate) fn next_token_is_data_type(&mut self) -> bool {
        if !self.has_next_token() {
            false
        } else {
            let first_token = self.get_next_token();

            match first_token {
                Token::Data => {
                    if !self.has_next_token() {
                        self.unget_next_token(first_token);
                        false
                    } else {
                        let second_token = self.get_next_token();

                        match second_token {
                            Token::Type => {
                                self.unget_next_token(second_token);
                                self.unget_next_token(first_token);
                                true
                            }
                            _ => {
                                self.unget_next_token(second_token);
                                self.unget_next_token(first_token);
                                false
                            }
                        }
                    }
                }
                _ => {
                    self.unget_next_token(first_token);
                    false
                }
            }
        }
    }

    // 다음 토큰이 default인지
    pub(crate) fn next_token_is_default(&mut self) -> bool {
        if !self.has_next_token() {
            false
        } else {
            let first_token = self.get_next_token();

            match first_token {
                Token::Default => {
                    self.unget_next_token(first_token);
                    true
                }
                _ => {
                    self.unget_next_token(first_token);
                    false
                }
            }
        }
    }
}
