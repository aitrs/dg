use rand::{thread_rng, Rng};
pub enum DefinitionExpression {
    Number(i64),
    Sum(Box<DefinitionExpression>, Box<DefinitionExpression>),
    Difference(Box<DefinitionExpression>, Box<DefinitionExpression>),
    Product(Box<DefinitionExpression>, Box<DefinitionExpression>),
    Divide(Box<DefinitionExpression>, Box<DefinitionExpression>),
    Modulo(Box<DefinitionExpression>, Box<DefinitionExpression>),
    Random(Box<DefinitionExpression>, Box<DefinitionExpression>),
    Increment,
}

impl DefinitionExpression {
    pub fn run(&self, increment_value: usize) -> i64 {
        match self {
            Self::Difference(a, b) => a.run(increment_value) - b.run(increment_value),
            Self::Sum(a, b) => a.run(increment_value) + b.run(increment_value),
            Self::Product(a, b) => a.run(increment_value) * b.run(increment_value),
            Self::Divide(a, b) => a.run(increment_value) / b.run(increment_value),
            Self::Modulo(a, b) => a.run(increment_value) % b.run(increment_value),
            Self::Random(a, b) => {
                thread_rng().gen_range(a.run(increment_value)..b.run(increment_value))
            }
            Self::Number(n) => *n,
            Self::Increment => increment_value as i64,
        }
    }
}

pub struct Definition(String, DefinitionExpression);

impl Definition {
    pub fn run(&self, increment: usize) -> (String, i64) {
        (self.0.to_owned(), self.1.run(increment))
    }
}

pub type Definitions = Vec<Definition>;

peg::parser! {
    pub grammar definition_parser() for str {
        pub rule definitions() -> Definitions
            = l:(definition() ** delim()) { l }

        pub rule definition() -> Definition
            = i:ident() _ "=" _ d:sum() _ {
                Definition(i, d)
            }

        rule ident() -> String
            = w:$(['a'..='z' | 'A'..='Z' | '0'..='9']+) {
                w.to_string()
            }

        rule number() -> DefinitionExpression
            = n:$("-"?['0'..='9']+) { DefinitionExpression::Number(n.parse().unwrap() )}

        rule _
            = [' ']*

        rule delim() = _ ['\n']+ _

        rule sum() -> DefinitionExpression
            = l:product() _ "+" _ r:product() { DefinitionExpression::Sum(Box::new(l), Box::new(r)) }
            / l:product() _ "-" _ r:product() { DefinitionExpression::Difference(Box::new(l), Box::new(r)) }
            / l:product() _ "%" _ r:product() { DefinitionExpression::Modulo(Box::new(l), Box::new(r)) }
            / product()

        rule product() -> DefinitionExpression
            = l:atom() _ "*" _ r:atom() { DefinitionExpression::Product(Box::new(l), Box::new(r)) }
            / l:atom() _ "/" _ r:atom() { DefinitionExpression::Divide(Box::new(l), Box::new(r)) }
            / atom()

        rule atom() -> DefinitionExpression
            = number()
            / random()
            / increment()
            / "(" _ v:sum() _ ")" { v }

        rule random() -> DefinitionExpression
            = "random(" low:sum() ":" high:sum() ")" { DefinitionExpression::Random(Box::new(low), Box::new(high)) }

        rule increment() -> DefinitionExpression
            = "incr()" { DefinitionExpression::Increment }
    }
}
