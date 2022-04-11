use aoc_2021::input;

fn main() {
    let input = input!("7");
    let input: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();

    let data = (input.iter().min().copied().unwrap()..input.iter().max().copied().unwrap())
        .map(|x| input.iter().map(|&y| triangle((y - x).abs())).sum::<i64>())
        .min()
        .unwrap();

    dbg!(&data);
}

fn triangle(n: i64) -> i64 {
    (1..=n).sum()
}
