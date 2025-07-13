use crate::aoc::Solution;
use crate::day7::CircuitError::UninitializedInputs;
use regex::Regex;
use std::collections::HashMap;
use thiserror::Error;

pub fn solve(input: impl AsRef<str>) -> Solution {
    let parser = CircuitParser::new();
    let connections = input
        .as_ref()
        .lines()
        .map(|line| parser.parse_line(line))
        .collect::<Vec<_>>();

    let mut circuit = Circuit::new();
    circuit.build(&connections).unwrap();
    let part1 = circuit.values.get("a").unwrap();

    let mut circuit = Circuit::new();
    circuit.values.insert("b".into(), *part1);
    circuit.build(&connections).unwrap();
    let part2 = circuit.values.get("a").unwrap();

    Solution::new(7, part1, part2)
}

type WireID = String;

#[derive(PartialEq, Eq, Debug)]
enum Value {
    Wire(WireID),
    Const(u16),
}

impl From<&str> for Value {
    fn from(input: &str) -> Value {
        if let Ok(num) = input.parse::<u16>() {
            Value::Const(num)
        } else {
            Value::Wire(input.to_string())
        }
    }
}
impl From<u16> for Value {
    fn from(input: u16) -> Value {
        Value::Const(input)
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Op {
    Set(Value),
    And(Value, Value),
    Or(Value, Value),
    LShift(Value, u16),
    RShift(Value, u16),
    Not(Value),
}

#[derive(PartialEq, Eq, Debug)]
struct Conn {
    target: WireID,
    op: Op,
}

struct Circuit<'a> {
    values: HashMap<WireID, u16>,
    pending: HashMap<WireID, Vec<&'a Conn>>,
}

#[derive(Error, Debug)]
enum CircuitError {
    #[error("Uninitialized inputs {}", fmt_ids(.0))]
    UninitializedInputs(Vec<WireID>),
}

fn fmt_ids(ids: &[WireID]) -> String {
    ids.iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

impl<'a> Circuit<'a> {
    fn new() -> Self {
        let values = HashMap::new();
        let pending = HashMap::new();
        Self { values, pending }
    }

    fn build(&mut self, connections: &'a Vec<Conn>) -> Result<&Circuit, CircuitError> {
        for conn in connections {
            match self.try_op(conn) {
                Ok(result) => {
                    self.values.entry(conn.target.clone()).or_insert(result);
                    self.process_pending(&conn.target);
                }
                Err(UninitializedInputs(ids)) => {
                    for id in ids {
                        let pending = self.pending.entry(id).or_default();
                        pending.push(conn);
                    }
                }
            }
        }

        Ok(self)
    }

    fn try_op(&self, conn: &Conn) -> Result<u16, CircuitError> {
        match &conn.op {
            Op::Set(from_id) => match self.get_value(&from_id) {
                Ok(from) => Ok(from),
                Err(err) => Err(err),
            },
            Op::And(left_id, right_id) => match self.get_values(&left_id, &right_id) {
                Ok((left, right)) => Ok(left & right),
                Err(err) => Err(err),
            },
            Op::Or(left_id, right_id) => match self.get_values(&left_id, &right_id) {
                Ok((left, right)) => Ok(left | right),
                Err(err) => Err(err),
            },
            Op::LShift(from_id, n) => match self.get_value(&from_id) {
                Ok(from) => Ok(from << n),
                Err(err) => Err(err),
            },
            Op::RShift(from_id, n) => match self.get_value(&from_id) {
                Ok(from) => Ok(from >> n),
                Err(err) => Err(err),
            },
            Op::Not(from_id) => match self.get_value(&from_id) {
                Ok(from) => Ok(!from),
                Err(err) => Err(err),
            },
        }
    }

    fn process_pending(&mut self, id: &WireID) {
        let mut to_process = vec![id];

        while let Some(current) = to_process.pop() {
            if let Some(conns) = self.pending.remove(current) {
                for conn in conns {
                    if let Ok(result) = self.try_op(conn) {
                        self.values.insert(conn.target.clone(), result);
                        to_process.push(&conn.target);
                    }
                }
            }
        }
    }

    fn get_value(&self, value: &Value) -> Result<u16, CircuitError> {
        match value {
            Value::Wire(id) => self
                .values
                .get(id)
                .copied()
                .ok_or_else(|| UninitializedInputs(vec![id.clone()])),
            Value::Const(val) => Ok(*val),
        }
    }

    fn get_values(&self, left: &Value, right: &Value) -> Result<(u16, u16), CircuitError> {
        let left = self.get_value(left);
        let right = self.get_value(right);

        if let (Ok(left), Ok(right)) = (&left, &right) {
            return Ok((*left, *right));
        } else if let (Err(UninitializedInputs(left)), Err(UninitializedInputs(right))) =
            (&left, &right)
        {
            let mut out = Vec::new();
            out.extend(left.iter().cloned());
            out.extend(right.iter().cloned());
            return Err(UninitializedInputs(out));
        } else if let Err(left_err) = left {
            return Err(left_err);
        } else if let Err(right_err) = right {
            return Err(right_err);
        }

        unreachable!()
    }
}

struct CircuitParser {
    re_value: Regex,
    re_and: Regex,
    re_or: Regex,
    re_lshift: Regex,
    re_rshift: Regex,
    re_not: Regex,
}

impl CircuitParser {
    fn new() -> Self {
        CircuitParser {
            re_value: Regex::new(r"^(\d+|[a-z]+) -> ([a-z]+)$").unwrap(),
            re_and: Regex::new(r"^(\d+|[a-z]+) AND (\d+|[a-z]+) -> ([a-z]+)$").unwrap(),
            re_or: Regex::new(r"^(\d+|[a-z]+) OR (\d+|[a-z]+) -> ([a-z]+)$").unwrap(),
            re_lshift: Regex::new(r"^(\d+|[a-z]+) LSHIFT (\d+) -> ([a-z]+)$").unwrap(),
            re_rshift: Regex::new(r"^(\d+|[a-z]+) RSHIFT (\d+) -> ([a-z]+)$").unwrap(),
            re_not: Regex::new(r"^NOT (\d+|[a-z]+) -> ([a-z]+)$").unwrap(),
        }
    }
    fn parse_line(&self, line: &str) -> Conn {
        if let Some(cap) = self.re_value.captures(line) {
            return Conn {
                target: cap[2].into(),
                op: Op::Set(cap[1].into()),
            };
        }
        if let Some(cap) = self.re_and.captures(line) {
            return Conn {
                target: cap[3].into(),
                op: Op::And(cap[1].into(), cap[2].into()),
            };
        }
        if let Some(cap) = self.re_or.captures(line) {
            return Conn {
                target: cap[3].into(),
                op: Op::Or(cap[1].into(), cap[2].into()),
            };
        }
        if let Some(cap) = self.re_lshift.captures(line) {
            return Conn {
                target: cap[3].into(),
                op: Op::LShift(cap[1].into(), cap[2].parse().unwrap()),
            };
        }
        if let Some(cap) = self.re_rshift.captures(line) {
            return Conn {
                target: cap[3].into(),
                op: Op::RShift(cap[1].into(), cap[2].parse().unwrap()),
            };
        }
        if let Some(cap) = self.re_not.captures(line) {
            return Conn {
                target: cap[2].into(),
                op: Op::Not(cap[1].into()),
            };
        }
        dbg!(line);
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_CIRCUIT: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
1 AND y -> j";

    #[test]
    fn test_parse() {
        let parser = CircuitParser::new();
        let actual = SAMPLE_CIRCUIT
            .lines()
            .map(|line| parser.parse_line(line))
            .collect::<Vec<_>>();
        #[rustfmt::skip]
        let expected = vec![
            Conn{target: "x".into(), op: Op::Set(123.into())},
            Conn{target: "y".into(), op: Op::Set(456.into())},
            Conn{target: "d".into(), op: Op::And("x".into(), "y".into())},
            Conn{target: "e".into(), op: Op::Or("x".into(), "y".into())},
            Conn{target: "f".into(), op: Op::LShift("x".into(), 2)},
            Conn{target: "g".into(), op: Op::RShift("y".into(), 2)},
            Conn{target: "h".into(), op: Op::Not("x".into())},
            Conn{target: "i".into(), op: Op::Not("y".into())},
            Conn{target: "j".into(), op: Op::And(1.into(), "y".into())},
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_build() {
        let parser = CircuitParser::new();
        let connections = SAMPLE_CIRCUIT
            .lines()
            .map(|line| parser.parse_line(line))
            .collect::<Vec<_>>();
        let mut circuit = Circuit::new();
        circuit.build(&connections).unwrap();

        assert_eq!(circuit.values.get("d").unwrap(), &72);
        assert_eq!(circuit.values.get("e").unwrap(), &507);
        assert_eq!(circuit.values.get("f").unwrap(), &492);
        assert_eq!(circuit.values.get("g").unwrap(), &114);
        assert_eq!(circuit.values.get("h").unwrap(), &65412);
        assert_eq!(circuit.values.get("i").unwrap(), &65079);
        assert_eq!(circuit.values.get("x").unwrap(), &123);
        assert_eq!(circuit.values.get("y").unwrap(), &456);
    }
}
