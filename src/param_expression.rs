use nom::{double_s, alpha};
use nom::IResult::*;

use tree::pbus;
use params::BusParam;

/// simple calculator like expressions for param
/// allow to write simple equations with "bus" params

pub trait CalcParam: Send {
    fn calc(&self) -> f64;
    fn connect(&mut self, buses: &mut pbus::BusSystem) {}
}

enum CalcBinOp {
    Add,
    Sub,
    Mult,
}

/// a node in the AST
struct CalcNode {
    left: Box<CalcParam>,
    right: Box<CalcParam>,
    op: CalcBinOp,
}


impl CalcParam for CalcNode {
    fn calc(&self) -> f64 {
        match &self.op {
            CalcBinOp::Add => self.left.calc() + self.right.calc(),
            CalcBinOp::Sub => self.left.calc() - self.right.calc(),
            CalcBinOp::Mult => self.left.calc() * self.right.calc(),
        }
    }

    fn connect(&mut self, buses: &mut pbus::BusSystem) {
        self.left.connect(buses);
        self.right.connect(buses);
    }
}


/// a literal constant
struct CalcCst {
    val: f64,
}

impl CalcParam for CalcCst {
    fn calc(&self) -> f64 {
        self.val
    }
}

impl CalcParam for BusParam {
    fn calc(&self) -> f64 {
        self.value()
    }

    fn connect(&mut self, buses: &mut pbus::BusSystem) {
        self.connect_to_bus(buses)
    }
}

// parsing shit. largely inspired from the simple calculator nom example

fn parse_constant(value: f64) -> Box<CalcParam> {
    Box::new(CalcCst { val: value })
}

fn parse_bus_param(name: &str) -> Box<CalcParam> {
    Box::new(BusParam::NotConnected(name.to_string()))
}

// terminals
named!( constant_p<&str, Box<CalcParam>>, map!(ws!(double_s), parse_constant ));
named!( variable_p<&str, Box<CalcParam>>, map!(ws!(alpha), parse_bus_param));

named!( parens_p<&str, Box<CalcParam>>, ws!(delimited!(char!('('), expression_p, char!(')'))));

// terminated leme
named!(op_p<&str, Box<CalcParam>>, alt!( constant_p | variable_p | parens_p));

// achtung with op precedence ..
named!(factor_p<&str, Box<CalcParam> >, do_parse!(
    op: op_p >>
    rem: many0!(tuple!(char!('*'), factor_p)) >>
    (parse_exp(op, rem))
));

named!(expression_p<&str, Box<CalcParam> >, do_parse!(
    f: factor_p >>
    rem: many0!(tuple!( alt!(char!('+') | char!('-')), factor_p)) >>
    (parse_exp(f, rem))
));

fn parse_exp(left: Box<CalcParam>, rem: Vec<(char, Box<CalcParam>)>) -> Box<CalcParam> {
    rem.into_iter().fold(left, |acc, (op, expr)| match op {
        '*' => Box::new(CalcNode {
            left: acc,
            right: expr,
            op: CalcBinOp::Mult,
        }),
        '-' => Box::new(CalcNode {
            left: acc,
            right: expr,
            op: CalcBinOp::Sub,
        }),
        '+' => Box::new(CalcNode {
            left: acc,
            right: expr,
            op: CalcBinOp::Add,
        }),
        _ => panic!(format!("unknown operation : {}", op)),
    })
}

/// parse a string as useable CalcParam.
pub fn parse_param_expression(input: &str) -> Result<Box<CalcParam>, String> {
    match expression_p(input) {
        Done(_, expr) => Ok(expr),
        _ => Err("shit".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_param_parser() {
        let e = parse_param_expression("2. * 4. + 1.0").unwrap();
        assert_eq!(9.0, e.calc());

        assert_eq!(12.0, parse_param_expression("(1.0 + 2.0 )*4.0").unwrap().calc());


    }
}
