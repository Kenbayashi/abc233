fn main() {
    proconio::input! {
        x: i32,
        y: i32,
    }

    let mut ans = 0;

    while x + (ans * 10) < y {
        ans += 1;
    }

    println!("{}", ans);
}
