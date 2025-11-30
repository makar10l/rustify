fn main(){
    let mut user = String::new();

    println!("\t\tRustify installer<3\n\n");
    println!("Write your username: ");

    std::io::stdin().read_line(&mut user).expect("Error with getting username");
    println!("Installing...\n");

    std::fs::write("data/username", user.trim())
    .expect("Cannot write username to file");

    std::fs::create_dir(format!("/home/{}/.rustify", user.trim()))
    .expect("Cannot find user home dir");
    std::fs::create_dir(format!("/home/{}/.rustify/data", user.trim()))
    .expect("Cannot find user rustify dir");
    #[cfg(not(debug_assertions))]
    {
    match std::fs::copy("target/release/rustify", "/bin/rustify"){
        Err(err) => match err.kind(){
            std::io::ErrorKind::PermissionDenied => {
                println!("Please restart with sudo");
                return ();
            },
            _ => {return ()},
        },
        Ok(_) => 0,
    };
    }
    println!("Succesfull!");
}