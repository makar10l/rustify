fn main(){
    println!("\t\tRustify installer<3");
    println!("Installing...\n");
    match std::fs::copy("target/release/rustify", "/bin/.main_rustify"){
        Err(err) => match err.kind(){
            std::io::ErrorKind::PermissionDenied => {
                println!("Please restart with sudo");
                return ();
            },
            _ => {return ()},
        },
        Ok(_) => 0,
    };
    match std::fs::copy("target/release/starter", "/bin/rustify"){
        Err(err) => match err.kind(){
            std::io::ErrorKind::PermissionDenied => {
                println!("Please restart with sudo");
                return ();
            },
            _ => {return ()},
        },
        Ok(_) => 0,
    };
    println!("Succesfull!");
}