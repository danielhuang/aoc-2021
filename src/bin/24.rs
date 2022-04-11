#![feature(box_syntax)]

use aoc_2021::input;
use defaultmap::DefaultHashMap;
use itertools::Itertools;

fn main() {
    let input = input!("24");

    let mut counts = DefaultHashMap::new(0);

    println!("from z3 import *");
    println!("opt = Optimize()");

    println!("x0 = 0");
    println!("y0 = 0");
    println!("z0 = 0");
    println!("w0 = 0");

    for line in input.lines() {
        let (ins, val) = line.split_once(' ').unwrap();

        match ins {
            "inp" => {
                counts[val] += 1;
                println!(
                    "{}{} = BitVec('{}{}', 64)",
                    val, counts[val], val, counts[val]
                );
            }
            _ => {
                let (val1, val2) = val.split_once(' ').unwrap();

                counts[val1] += 1;
                println!(
                    "{}{} = BitVec('{}{}', 64)",
                    val1, counts[val1], val1, counts[val1]
                );

                if ins == "eql" {
                    println!(
                        "opt.add({}{} == If({}{} == {}, BitVecVal(1, 64), BitVecVal(0, 64)))",
                        val1,
                        counts[val1],
                        val1,
                        counts[val1] - 1,
                        match val2.parse::<i64>() {
                            Ok(x) => x.to_string(),
                            Err(_) => format!("{}{}", val2, counts[val2]),
                        }
                    );
                } else {
                    println!(
                        "opt.add({}{} == {}{} {} {})",
                        val1,
                        counts[val1],
                        val1,
                        counts[val1] - 1,
                        match ins {
                            "add" => "+",
                            "mul" => "*",
                            "div" => "/",
                            "mod" => "%",
                            _ => unreachable!(),
                        },
                        match val2.parse::<i64>() {
                            Ok(x) => x.to_string(),
                            Err(_) => format!("{}{}", val2, counts[val2]),
                        }
                    );
                }

                if ins == "div" || ins == "mod" {
                    println!("opt.add({}{} != BitVecVal(0, 64))", val2, counts[val2]);
                }
            }
        }
    }

    println!("opt.add(z{} == 0)", counts["z"]);
    for n in 1..=14 {
        println!("opt.add(w{} > 0)", n);
        println!("opt.add(w{} < 10)", n);
    }
    println!(
        "opt.minimize({})",
        (1..=14)
            .map(|x| format!("w{} * {}", x, 10i64.pow(14 - x)))
            .collect_vec()
            .join(" + ")
    );

    println!("opt.check()");
    println!("m = opt.model()");
    println!("f = ''");
    for i in 1..=14 {
        println!("f += str(m[w{}])", i)
    }
    println!("f")
}
