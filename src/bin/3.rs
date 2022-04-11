use aoc_2021::input;
use itertools::Itertools;

fn main() {
    let input = input!("3");
    let input = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| match x {
                    '1' => true,
                    '0' => false,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut partials = vec![];
    for i in 0..(input[0].len()) {
        let partial = input.iter().map(|x| x[i]).collect_vec();
        partials.push(partial);
    }

    let counts = partials
        .iter()
        .map(|x| (x.len(), x.iter().filter(|&&x| x).count()))
        .collect_vec();
    dbg!(&counts);

    let bin: String = counts
        .iter()
        .map(|&(total, x)| x >= total / 2)
        .map(|x| if x { "1" } else { "0" })
        .collect();

    dbg!(&bin);

    let num = usize::from_str_radix(&bin, 2).unwrap();
    dbg!(num);

    let bin2: String = counts
        .iter()
        .map(|&(total, x)| x >= total / 2)
        .map(|x| if x { "0" } else { "1" })
        .collect();

    dbg!(&bin2);

    let num2 = usize::from_str_radix(&bin2, 2).unwrap();
    dbg!(num2);

    dbg!(num * num2);

    let mut input2 = input.clone();
    for i in 0..(input[0].len()) {
        input2 = filter(&input2, i, false);
        dbg!(input2.iter().map(|x| fmt(x)).collect_vec());
    }
    let num3 = get_num(&input2[0]);

    let mut input2 = input.clone();
    for i in 0..(input[0].len()) {
        input2 = filter(&input2, i, true);
        dbg!(input2.iter().map(|x| fmt(x)).collect_vec());
    }
    let num4 = get_num(&input2[0]);

    dbg!(num3 * num4);
}

fn filter(input: &[Vec<bool>], i: usize, invert: bool) -> Vec<Vec<bool>> {
    if input.len() == 1 {
        return input.to_vec();
    }

    let mut partials = vec![];
    for i in 0..(input[0].len()) {
        let partial = input.iter().map(|x| x[i]).collect_vec();
        partials.push(partial);
    }

    let counts = partials
        .iter()
        .map(|x| (x.len(), x.iter().filter(|&&x| x).count()))
        .collect_vec();
    let (total, count) = counts[i];

    let count_true = count;
    let count_false = total - count;

    let bit = count_true >= count_false;

    dbg!(&bit);

    input
        .iter()
        .filter(|x| x[i] == bit ^ invert)
        .map(|x| x.to_vec())
        .collect_vec()
}

fn fmt(data: &[bool]) -> String {
    data.iter().map(|&x| if x { "1" } else { "0" }).collect()
}

fn get_num(data: &[bool]) -> usize {
    usize::from_str_radix(&fmt(data), 2).unwrap()
}
