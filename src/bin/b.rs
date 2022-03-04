use proconio::marker::Chars;

fn main() {
    proconio::input! {
        l: usize,
        r: usize,
        s: Chars,
    }

    let length = s.len();

    for count in 1..=length {
        if (l <= count) && (count <= r) {
            // 内側
            print!("{}", s[r - count + l - 1]);
        } else {
            // 外側
            print!("{}", s[count - 1]);
        }
    }
}
