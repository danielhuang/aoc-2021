use aoc_2021::input;
use defaultmap::DefaultHashMap;
use itertools::Itertools;

fn main() {
    let input = input!("6");
    let input = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    let mut env = DefaultHashMap::new(0);
    for &val in input.iter() {
        env[val] += 1;
    }

    for _ in 0..80 {
        env = tick(&env);
    }

    dbg!(env.values().sum::<usize>());

    for _ in 0..(256 - 80) {
        env = tick(&env);
    }

    dbg!(env.values().sum::<usize>());
}

fn tick(env: &DefaultHashMap<i64, usize>) -> DefaultHashMap<i64, usize> {
    let mut new_env = DefaultHashMap::new(0);
    for (&timer, &count) in env.iter() {
        if timer > 0 {
            new_env[timer - 1] += count;
        } else {
            new_env[6] += count;
            new_env[8] += count;
        }
    }
    new_env
}
