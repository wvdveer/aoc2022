use std::fs;

#[derive(Clone)]
struct Rational {
    numerator: Vec<i64>,
    denominator: i64
}

impl Rational {
    fn new() -> Rational {
        Rational { numerator: Vec::new(), denominator: 1 }
    }

    fn reduce(self: &Self) -> Rational {
        let mut hcf: i64 = self.denominator;
        for i in 0..self.numerator.len() {
            if hcf > 1 {
                hcf = gcd( hcf, self.numerator[i]);
            }
        }
        let mut rslt = self.clone();
        if hcf > 1 {
            for i in 0..rslt.numerator.len() {
                rslt.numerator[i] = rslt.numerator[i] / hcf;
            }
            rslt.denominator = rslt.denominator / hcf;
        };
        rslt
    }
}


fn gcd(a: i64, b: i64) -> i64 { 
    let mut gcd = 1;
    for i in 2..20 {
        if (a % i) == 0 && (b % i) == 0 {
            gcd = i;
        }
    }
    return gcd;
}

#[derive(Clone)]
struct Monkey {
    name: [char;4],
    is_num: bool,
    op1name: [char;4],
    op2name: [char;4],
    op: char,
    is_eval: bool,
    value: Rational
}

impl Monkey {
    fn parse(src: &str) -> Monkey {
        let parts: Vec<&str> = src.split(": ").collect(); 
        let name = parse_name(&parts[0]);
        let op1name: [char;4];
        let op2name: [char;4];
        let op: char;
        let is_num: bool;
        let mut value: Rational = Rational::new();
        if ('0'..='9').contains(&parts[1].chars().nth(0).unwrap()) {
            op1name = [' ';4];
            op = ' ';
            op2name = [' ';4];
            is_num = true;
            value.numerator.push(parts[1].parse::<i64>().unwrap());
        } else {
            op1name = parse_name(&parts[1][0..4]);
            op = parts[1][5..6].chars().nth(0).unwrap();
            op2name = parse_name(&parts[1][7..11]);
            is_num = false;
        }
        if parts[0] == "humn" {
            value.numerator = Vec::new();
            value.numerator.push(0);
            value.numerator.push(1);
        }
        Monkey { name: name, is_num: is_num, op1name: op1name, op2name: op2name, op: op, is_eval: is_num, value: value }
    }

}

fn parse_name(src: &str) -> [char;4] {
    let mut rslt: [char;4] = [' ';4];
    for idx in 0..4 {
        rslt[idx] = src.chars().nth(idx).unwrap();
    };
    rslt    
}

fn get_monkey_idx(name: [char;4], monkeys: &Vec<Monkey>) -> usize {
    for i in 0..monkeys.len() {
        if monkeys[i].name == name {
            return i;
        }
    }
    return 99999;
}

fn calc(op1val: &Rational, op2val: &Rational, op: char) -> Rational {
    if op == '+' || op == '-' {
        let mut rslt: Rational = Rational::new();
        while rslt.numerator.len() < op1val.numerator.len() || rslt.numerator.len() < op2val.numerator.len() {
            rslt.numerator.push(0);
        }
        rslt.denominator = op1val.denominator * op2val.denominator;
        for idx in 0..rslt.numerator.len() {
            let op1coeff: i64 = if idx >= op1val.numerator.len() { 0 } else { op1val.numerator[idx] * op2val.denominator };
            let op2coeff: i64 = if idx >= op2val.numerator.len() { 0 } else { op2val.numerator[idx] * op1val.denominator };
            let coeff: i64 = if op == '+' { op1coeff + op2coeff } else { op1coeff - op2coeff };
            rslt.numerator[idx] = coeff;
        }
        return rslt;
    } else if op == '*' {
        let mut rslt: Rational = Rational::new();
        while rslt.numerator.len() < (op1val.numerator.len() + op2val.numerator.len() - 1) {
            rslt.numerator.push(0);
        }
        for op1idx in 0..op1val.numerator.len() {
            for op2idx in 0..op2val.numerator.len() {
                rslt.numerator[ op1idx + op2idx ] = rslt.numerator[ op1idx + op2idx ] + ( op1val.numerator[op1idx] * op2val.numerator[op2idx] );
            }    
        }
        rslt.denominator = op1val.denominator * op2val.denominator;
        return rslt;
    } else {
        if op2val.numerator.len() != 1 {
            panic!("Need negative coeffs");
        }
        let mut rslt = op1val.clone();
        for i in 0..rslt.numerator.len() {
            rslt.numerator[i] = rslt.numerator[i] * op2val.denominator;
        }
        rslt.denominator = rslt.denominator * op2val.numerator[0];
        return rslt;
    };
}

fn evaluate(name: [char;4], monkeys: & mut Vec<Monkey>) -> Rational {
    let idx = get_monkey_idx(name, monkeys);
    if idx == 99999 {
        panic!("monkey not found: {}{}{}{}", name[0], name[1], name[2], name[3]);
    }
    let mut this_monkey: Monkey = monkeys[idx].clone();
    if !this_monkey.is_eval {
        let op1val: Rational = evaluate(this_monkey.op1name, monkeys).reduce();
        let op2val: Rational = evaluate(this_monkey.op2name, monkeys).reduce();
        this_monkey.value = calc(&op1val, &op2val, this_monkey.op);
        this_monkey.is_eval = true;                        
    }
    monkeys[idx] = this_monkey.clone();
    return this_monkey.value;
}

fn main() {
    let file_path: String = "data/input.txt".to_string();
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect(); 

    let mut monkeys: Vec<Monkey> = lines.iter().map(|l|{
        Monkey::parse(l)
    }).collect();

    let root_monkey_idx = get_monkey_idx(parse_name("root"), &monkeys);
    monkeys[root_monkey_idx].op = '-';

    let coeffs = evaluate(parse_name("root"), &mut monkeys);

    for i in 0..coeffs.numerator.len() {
        print!("{} {}\r\n", i, coeffs.numerator[i]);
    }
    print!("denom: {}\r\n", coeffs.denominator);

    print!("{}", coeffs.numerator[0] / (0 -coeffs.numerator[1]));
}
