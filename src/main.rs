#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, PrecClimber};
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CalcParser;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use pest::prec_climber::Operator;
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(operation_add, Left) | Operator::new(operation_subtract, Left),
            Operator::new(operation_multiply, Left) | Operator::new(operation_divide, Left),
            Operator::new(operation_or, Left),
            Operator::new(operation_and, Left),
        ])
    };
}

#[derive(Debug)]
pub enum Operator {
    /// Numeric multiplication
    Multiply,

    /// Numeric division
    Divide,

    /// Numeric addition
    Add,

    /// Numeric subtraction
    Subtract,

    /// Concatenation of character sequences
    Concat,

    /// Logical and
    And,

    /// Logical or
    Or,
}

impl Operator {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "*" => Self::Multiply,
            "/" => Self::Divide,
            "+" => Self::Add,
            "-" => Self::Subtract,
            "||" => Self::Concat,
            "AND" => Self::And,
            "OR" => Self::Or,
            _ => unreachable!(format!("Operator: symbol {} not supported", s)),
        }
    }
}

#[derive(Debug)]
enum AstNode {
    Identifier {
        s: String,
    },
    Expr {
        left: Box<AstNode>,
        op: Operator,
        right: Box<AstNode>,
    },
}

fn parse_value(expression: Pairs<Rule>) -> AstNode {
    // eprintln!("parse_value: {:#?}", expression);
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| {
            // eprintln!("primary: {:#?}", pair);
            match pair.as_rule() {
                Rule::num => AstNode::Identifier {
                    s: pair.as_str().to_string(),
                },
                Rule::expr => parse_value(pair.into_inner()),
                _ => unreachable!(),
            }
        },
        |left: AstNode, op: Pair<Rule>, right: AstNode| {
            // eprintln!("infix: {:#?}", op);
            match op.as_rule() {
                Rule::operation_add
                | Rule::operation_subtract
                | Rule::operation_multiply
                | Rule::operation_divide
                | Rule::operation_and
                | Rule::operation_or => AstNode::Expr {
                    left: Box::new(left),
                    op: Operator::from_str(op.as_str()),
                    right: Box::new(right),
                },
                _ => unreachable!(),
            }
        },
    )
}

fn main() {
    let tokens = CalcParser::parse(Rule::calculation, "1 or 2 and 3").unwrap();
    let ast = parse_value(tokens);

    println!("{:#?}", ast);
}
