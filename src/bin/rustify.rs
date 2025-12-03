fn main() {
    let mut void = String::new();
    println!("{}", files::get::get_logo().expect("wtf"));
    std::io::stdin().read_line(&mut void).expect("IDK");
}
