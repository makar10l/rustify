fn main(){
    let user = std::env::var("USER").expect("dfdj");
    // match std::fs::create_dir(format!("/home/{}/.rustify", user)){
    //     Err(err) => {
    //         match err.kind(){
    //             std::io::ErrorKind::AlreadyExists => ffiles::create::create_data(),
    //             _ => (),
    //         }
    //     },
    //     Ok(_) => (),
    // }
    files::create::create_data();
    std::process::Command::new("alacritty")
    .arg("-o")
    .arg("window.dimensions.columns=100")
    .arg("-o")
    .arg("window.dimensions.lines=30")
    .arg("-e")
    .arg(format!("/home/{}/.rustify/bin/rustify", user))
    .spawn().expect("FUCK SONIA");
    std::process::Command::new("killactive").spawn().expect("idk");
    
}