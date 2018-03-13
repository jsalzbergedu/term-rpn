use std::env::args;
use std::process::exit;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Copy, Clone, Debug)]
enum NumberType {
    Integer(i64),
    Float(f64),
}

impl NumberType {
    fn from_str(s: &str) -> Result<NumberType, String> {
        use NumberType::Integer;
        use NumberType::Float;
        if s.contains(".") {
            Ok(Float(s.parse().map_err(|_x| format!("{} misidentified as floating point number.", s))?))
        } else {
            Ok(Integer(s.parse().map_err(|_x| format!("{} misidentified as integer number", s))?))
        }
    }

    fn run_proc<T, S>(self, other: Self, iproc: T, fproc: S) -> Self
        where T: Fn(i64, i64) -> i64, S: Fn(f64, f64) -> f64 {
        use NumberType::Integer;
        use NumberType::Float;
        match self {
            Integer(i) => {
                match other {
                    Integer(o) => Integer(iproc(i, o)),
                    Float(o) => Float(fproc(i as f64, o)),
                }
            },
            Float(f) => {
                match other {
                    Integer(o) => Float(fproc(f, o as f64)),
                    Float(o) => Float(fproc(f, o)),
                }
            },
        }
    }

    fn add(self, other: Self) -> Self {
        self.run_proc(other, |i, o| i + o, |f, o| f + o)
    }

    fn sub(self, other: Self) -> Self {
        self.run_proc(other, |i, o| i - o, |f, o| f - o)
    }

    fn mult(self, other: Self) -> Self {
        self.run_proc(other, |i, o| i * o, |f, o| f * o)
    }

    fn div(self, other: Self) -> Self {
        self.run_proc(other, |i, o| i / o, |f, o| f / o)
    }

    fn as_string(self) -> String {
        use NumberType::Integer;
        use NumberType::Float;
        match self {
            Integer(i) => format!("{}", i),
            Float(f) => format!{"{}", f},
        }
    }
}

impl Display for NumberType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.as_string())
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "")
    }
}

#[derive(Debug)]
enum StackItem {
    Num(NumberType),
    Op(Operator),
}

impl Display for StackItem {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use StackItem::*;
        match self {
            &Num(ref n) => n.fmt(f),
            &Op(ref o) => o.fmt(f),
        }
    }
}

#[derive(Debug)]
struct Stack {
    stack: Vec<StackItem>,
}

impl Stack {
    fn new() -> Stack {
        Stack { stack: Vec::new() }
    }
}

trait Pushable {
    fn push(&mut self, item: StackItem);
}

impl Pushable for Stack {
    fn push(&mut self, item: StackItem) {
        self.stack.push(item);
    }
}

trait AddItemAble: Pushable {
    fn add_item(&mut self, item: &str) -> Result<(), String> {
        use StackItem::Num;
        use StackItem::Op;
        use Operator::*;
        match item.parse::<char>() {
            Ok(c) => {
                match c {
                    'a' | 's' | 'm' | 'd' => (),
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => (),
                    _ => return Err(format!("{} is not an operator or number", c)),
                }
            },
            _ => (),
        }
        let item = match item {
            "a" => Op(Add),
            "s" => Op(Subtract),
            "m" => Op(Multiply),
            "d" => Op(Divide),
            _ => Num(NumberType::from_str(item)?),
        };
        self.push(item);
        Ok(())
    }
}

impl AddItemAble for Stack {}

trait Poppable {
    fn pop(&mut self) -> Result<StackItem, String>;
}

impl Poppable for Stack {
    fn pop(&mut self) -> Result<StackItem, String> {
        self.stack.pop().ok_or("Stack underflow".to_owned())
    }
}

trait Operable: Pushable + Poppable {}

impl Operable for Stack {}

trait ProcRunnable: Operable {
    fn op_run<T>(&mut self, token: Operator, nproc: T) -> Result<(), String>
        where T: Fn(NumberType, NumberType) -> NumberType {
        use StackItem::Op;
        use StackItem::Num;
        let top = self.pop()?;
        match top {
            Op(op) if op == token => {
                let snd = match self.pop()? {
                    Num(num) => num,
                    _ => return Err("Attempted to add non-numbers".to_owned()),
                };
                let fst = match self.pop()? {
                    Num(num) => num,
                    _ => return Err("Attempted to add non-numbers".to_owned()),
                };
                self.push(Num(nproc(fst, snd)));
            }
            Op(op) => {
                self.push(Op(op));
            },
            _ => self.push(top),
        }
        Ok(())
    }
}

impl ProcRunnable for Stack {}

trait Addable: ProcRunnable {
    fn add(&mut self) -> Result<(), String> {
        self.op_run(Operator::Add, |n, o| n.add(o))
    }
}

impl Addable for Stack {}

trait Subabble: ProcRunnable {
    fn sub(&mut self) -> Result<(), String> {
        self.op_run(Operator::Subtract, |n, o| n.sub(o))
    }
}

impl Subabble for Stack {}

trait Multable: ProcRunnable {
    fn mult(&mut self) -> Result<(), String> {
        self.op_run(Operator::Multiply, |n, o| n.mult(o))
    }
}

impl Multable for Stack {}

trait Divable: ProcRunnable {
    fn div(&mut self) -> Result<(), String> {
        self.op_run(Operator::Divide, |n, o| n.div(o))
    }
}

impl Divable for Stack {}

trait Reducable: Addable + Subabble + Multable + Divable {
    fn reduce(&mut self) -> Result<(), String> {
        self.add()?;
        self.sub()?;
        self.mult()?;
        self.div()?;
        Ok(())
    }
}

impl Reducable for Stack {}

static USAGE: &str = "An RPN calculator.
Supply arguments like this: 
rpn 1 2 3 a a
Operators: a (+) s (-) m (*) d (/)
Possible aruments include floating point number and integers.";

fn exit_with(msg: &str, code: i32) -> ! {
    eprintln!("Error: {}", msg);
    exit(code);
}

fn main() {
    let mut stack = Stack::new();
    let mut args = args().peekable();
    args.next();
    match args.peek().map(|s| &s[..]) {
        Some("-h") | Some("--help") | None => {
            exit_with(USAGE, 1);
        }
        _ => {
            for argument in args {
                let _ = stack.add_item(&argument).map_err(|s| exit_with(&s, 1));
                let _ = stack.reduce().map_err(|s| exit_with(&s, 1));
            }
            if stack.stack.len() > 1 { 
                let mut out_str = String::with_capacity(20);
                for item in &stack.stack[0..(stack.stack.len() - 1)] {
                    out_str += &format!("{} ", item);
                }
                println!("{}{}", out_str, stack.stack[stack.stack.len() - 1]);
            } else {
                println!("{}", stack.stack[0]);
            }
        }
    }
}
