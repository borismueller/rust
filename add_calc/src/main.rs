use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 4 {
        println!("not enough args :/");
    } else {
        if args[1].parse::<f64>().is_ok() && args[3].parse::<f64>().is_ok() {
            match args[2].as_ref() {
                "+" => println!("result {}", add(args[1].parse::<f64>().unwrap(), args[3].parse::<f64>().unwrap())),
                "*" => println!("result {}", multiply(args[1].parse::<f64>().unwrap(), args[3].parse::<f64>().unwrap())), //you need "*" for this to work in my terminal
                "." => println!("result {}", multiply(args[1].parse::<f64>().unwrap(), args[3].parse::<f64>().unwrap())),
                "-" => println!("result {}", subtract(args[1].parse::<f64>().unwrap(), args[3].parse::<f64>().unwrap())),
                "/" => println!("result {}", divide(args[1].parse::<f64>().unwrap(), args[3].parse::<f64>().unwrap())),
                _   => println!("did not recognize operator, {}", args[2]),
            }
        }
    }
}

fn divide(num1: f64, num2: f64) -> f64 {
    if num2 == 0.0 {
        panic!("don't divide by 0 :/");
    }
    let mut result: f64 = 0.0;
    let mut num = 0.0;
    while num1 + negate(num) > 0.0 {
        num += num2;
        result += 1.0;
    } if num1 + negate(num) < 0.0 {
        panic!("not a whole number ");
    } else {
        result
    }
}

fn subtract(num1: f64, num2: f64) -> f64 {
    add(num1, negate(num2))
}

fn negate(num: f64) -> f64 {
    let mut min = -1000000000.0; //should be std::f64::MIN but that takes too long
    //this is pretty ugly, but with just one loop everything takes forever
    while min + num < 0.0 {
        min += 10000000.0;
    }
    while min + num != 0.0 {
        min -= 1.0;
    }

    min
}

fn multiply(num1: f64, num2: f64) -> f64 {
    let mut result: f64 = 0.0;
    for _ in 0..num2.round() as i64 {
        result += add(num1, num2);
    }
    result
}

fn add(num1: f64, num2: f64) -> f64 {
    num1 + num2
}
