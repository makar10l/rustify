fn copy_build(target : &str, path : &str){
    match std::fs::copy(target, path){
        Err(_err) => match _err.kind(){
            std::io::ErrorKind::PermissionDenied => {
                println!("Please restart with sudo");
                ()
            },
            _err => {()},
        },
        Ok(_) => (),
    };
}

fn main(){
    println!("\t\tRustify installer<3");
    println!("Installing...\n");
    //copying starter
    copy_build("target/release/starter", "/bin/rustify");
    //copying rustify
    copy_build("target/release/rustify", "/bin/.main_rustify");
    println!("Succesfull!");
}