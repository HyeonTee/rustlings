fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(n) = option {
        res += n;
    }

    println!("{res}");
}
