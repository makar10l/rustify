fn main(){
    let user = std::env::var("USER").expect("dfdj");
    files::create::create_data();
    std::process::Command::new("alacritty")
    .arg("-e")
    .arg(format!("/home/{}/.rustify/bin/rustify", user))
    .spawn().expect("FUCK SONIA");
}