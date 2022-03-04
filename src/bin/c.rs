use std::io;
use std::io::Read;

fn main() {
    let n = read();
    let x = read();

    let table = (0..n).into_iter()
                      .map(|_| get_bag())
                      .collect::<Vec<Vec<usize>>>();

    let ans = dfs(&table, 0, table.len(), x).into_iter()
                                         .fold(0, |count, result| count + if x == result {1} else {0});

    println!("{:?}", ans);
}

fn read() -> usize {
    io::stdin().bytes()
               .map(|c| c.unwrap() as char)
               .skip_while(|c| c.is_whitespace())
               .take_while(|c| !c.is_whitespace())
               .collect::<String>()
               .parse::<usize>()
               .ok()
               .unwrap()
}

fn get_bag() -> Vec<usize> {
    let size = read();

    (0..size).into_iter()
             .map(|_| read())
             .collect::<Vec<usize>>()
}

fn dfs(table: &Vec<Vec<usize>>, count: usize, count_max: usize, x: usize) -> Vec<usize> {
    if count == count_max {
        return vec![1];
    }

    table[count].iter()
                .flat_map(|&ball| dfs(table, count + 1, count_max, x).into_iter()
                                                                     .map(|num| num.saturating_mul(ball))
                                                                     .filter(|&num| num <= x)
                                                                     .collect::<Vec<usize>>())
                .collect::<Vec<usize>>()
}
