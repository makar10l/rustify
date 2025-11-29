fn main(){
    let mut user = String::new();

    println!("\t\tRustify installer<3\n\n");
    println!("Write your username: ");

    std::io::stdin().read_line(&mut user).expect("Error with getting username");
    println!("Installing...\n");

    std::fs::create_dir(format!("/home/{}/.rustify", user.trim()))
    .expect("Cannot find user home dir");
    // std::fs::rename("data", format!("/home/{}/.rustify/data", user.trim()))
    // .expect("Cannot find rustify dir");

    match std::fs::rename("target/release/rustify", "/bin"){
        Err(err) => match err.kind(){
            std::io::ErrorKind::PermissionDenied => {
                println!("Please restart with sudo");
                return ();
            },
            _ => return (),
        },
        Ok(()) => 0,
    };
    println!("Succesfull!");
}