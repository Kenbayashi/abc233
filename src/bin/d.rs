use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let ans = (0..n).combinations_with_replacement(2)
                    .into_iter()
                    .map(|comb| (comb[0], comb[1]))
                    .map(|(l, r)| (&a[l..=r]).iter().sum::<i64>())
                    .fold(0, |count, ans| count + if k == ans {1} else {0});

    println!("{}", ans);
}
