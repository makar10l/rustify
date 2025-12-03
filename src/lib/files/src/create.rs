pub fn create_data(){
    let user = std::env::var("USER").expect("Error with getting username");
    exits(std::fs::create_dir(format!("/home/{}/.rustify", user)));
    exits(std::fs::create_dir(format!("/home/{}/.rustify/data", user)));
    exits(std::fs::create_dir(format!("/home/{}/.rustify/bin", user)));
    std::fs::copy("/bin/.main_rustify", format!("/home/{}/.rustify/bin/rustify", user)).expect("Error: cannot move bin");
}

fn exits(result : Result<(), std::io::Error>){
    match result{
        Err(err) => {
            match err.kind(){
                std::io::ErrorKind::AlreadyExists => (),
                _ => (),
            }
        },
        Ok(_) => (),
    };
}