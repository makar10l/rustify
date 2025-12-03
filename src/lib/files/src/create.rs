use std::io::Write;
pub fn create_data(){
    let user = std::env::var("USER").expect("Error with getting username");
    exits(std::fs::create_dir(format!("/home/{}/.rustify", user)));
    exits(std::fs::create_dir(format!("/home/{}/.rustify/data", user)));
    exits(std::fs::create_dir(format!("/home/{}/.rustify/bin", user)));
    logo(format!("/home/{}/.rustify/data/logo.txt", user)).unwrap();
    std::fs::copy("/bin/.main_rustify", format!("/home/{}/.rustify/bin/rustify", user)).expect("Error: cannot move bin");
}
pub fn remove_data(){
    let user = std::env::var("USER").expect("Error with getting username");
    std::fs::remove_dir_all(format!("/home/{}/.rustify", user)).expect("Cannot remove data");
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
fn logo(path : String) -> Result<(), std::io::Error>{
    let mut file = match std::fs::File::create(path){
        Err(err) => {println!("Cannot create logo.txt"); return Err(err)},
        Ok(descriptor) => descriptor, 
    };
    let logo = std::fs::read_to_string("data/logo.txt").expect("Cannot read logo");
    write!(file, "{}",logo).expect("Cannot write logo");
    Ok(())
}