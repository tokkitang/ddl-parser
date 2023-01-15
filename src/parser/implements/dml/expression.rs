use std::error::Error;

use crate::errors::predule::ParsingError;
use crate::lexer::predule::Token;
use crate::parser::predule::Parser;
use crate::parser::predule::ParserContext;

impl Parser {
    pub(crate) fn parse_expression(
        &mut self,
        context: ParserContext,
    ) -> Result<String, Box<dyn Error + Send>> {
        if !self.has_next_token() {
            return Err(ParsingError::boxed("E0201 need more tokens"));
        }

        let mut expression = String::new();

        while self.has_next_token() {
            let current_token = self.get_next_token();

            match current_token {
                Token::Operator(operator) => {
                    expression.push_str(operator.to_string().as_str());
                }
                Token::Not => {
                    expression.push_str("NOT");
                }
                Token::Integer(integer) => {
                    let string = integer.to_string();
                    expression.push_str(string.as_str());
                }
                Token::Float(float) => {
                    let string = float.to_string();
                    expression.push_str(string.as_str());
                }
                Token::String(string) => {
                    expression.push_str(string.as_str());
                }
                Token::Boolean(boolean) => {
                    let string = boolean.to_string();
                    expression.push_str(string.as_str());
                }
                Token::Null => {
                    expression.push_str("NULL");
                }
                Token::LeftParentheses => {
                    if !self.has_next_token() {
                        return Err(ParsingError::boxed("E0214 need more tokens"));
                    }

                    let second_token = self.get_next_token();

                    match second_token {
                        Token::Select => {
                            return Err(ParsingError::boxed("Select not supported in expression"));
                        }
                        _ => {
                            self.unget_next_token(second_token);
                            self.unget_next_token(current_token);
                            let string = self.parse_parentheses_expression(context.clone())?;
                            expression.push_str(string.as_str());
                        }
                    }
                }
                Token::RightParentheses => {
                    self.unget_next_token(current_token);
                    break;
                }
                Token::Comma => {
                    self.unget_next_token(current_token);
                    break;
                }
                _ => {
                    break;
                }
            }
        }

        Ok(expression)
    }

    /**
     * 소괄호연산자, 혹은 리스트 파싱
    parenexpr ::= '(' expression ')'
    parenexpr ::= '(' 1, 2, 3 ')'
    */
    pub(crate) fn parse_parentheses_expression(
        &mut self,
        context: ParserContext,
    ) -> Result<String, Box<dyn Error + Send>> {
        let context = context.set_in_parentheses(true);

        if !self.has_next_token() {
            return Err(ParsingError::boxed("E0203 need more tokens"));
        }

        // ( 삼킴
        let current_token = self.get_next_token();

        if current_token != Token::LeftParentheses {
            return Err(ParsingError::boxed(format!(
                "expected left parentheses. but your input is {:?}",
                current_token
            )));
        }

        if !self.has_next_token() {
            return Err(ParsingError::boxed("E0204 need more tokens"));
        }

        // 표현식 파싱
        let expression = self.parse_expression(context.clone())?;

        if !self.has_next_token() {
            return Err(ParsingError::boxed("E0205 need more tokens"));
        }

        // ) 삼킴
        let current_token = self.get_next_token();

        match current_token {
            // 우선순위 연산자
            Token::RightParentheses => {
                let expression = format!("({})", expression);

                Ok(expression.into())
            }
            _ => Err(ParsingError::boxed(format!(
                "expected right parentheses. but your input is {:?}",
                current_token
            ))),
        }
    }
}
