use std::env;
use std::process;

#[derive(Debug)]
struct Calculator {
    first: f32,
    second: f32,
    operation: String,
}

impl Calculator {
    pub fn new(args: &[String]) -> Result<Calculator, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let first = args[1]
            .clone()
            .trim()
            .parse()
            .expect("Error in parsing input");
        println!("{:?}", &args);
        let operation = args[2].clone();
        let second = args[3]
            .clone()
            .trim()
            .parse()
            .expect("Error in parsing input");

        Ok(Calculator {
            first,
            second,
            operation,
        })
    }

    pub fn sum(&self) -> f32 {
        self.first + self.second
    }

    pub fn subtract(&self) -> f32 {
        self.first - self.second
    }

    pub fn multiply(&self) -> f32 {
        self.first * self.second
    }

    pub fn divide(&self) -> f32 {
        self.first / self.second
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let calculator = Calculator::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let result: f32;
    match calculator.operation.as_str() {
        "+" => result = calculator.sum(),
        "-" => result = calculator.subtract(),
        "*" | "X" | "x" => result = calculator.multiply(),
        "/" => result = calculator.divide(),
        _ => {
            println!("Operation not managed: {}", calculator.operation);
            process::exit(1);
        }
    }

    println!("The result is {}", result);
}
