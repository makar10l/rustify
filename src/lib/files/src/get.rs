use std::io::{self, Write};
pub fn get_music_path() -> String{
    let username = std::env::var("USER").expect("cannot identificate you");
    let path_file = std::fs::read_to_string(
        format!("/home/{}/.rustify/data/path_music",username)
    ).expect("Reading music path failed!"); 
    if path_file.is_empty(){
        println!("
            Your path to music is empty, please write ABSOLUTE path to your music <3:
        ");
        let mut path = String::new();
        io::stdin().read_line(&mut path).expect("OS error");
        path = path.trim().to_string();
        write!(std::fs::File::create(format!("/home/{}/.rustify/data/path_music",user)).unwrap(), "{}", path).expect("im stupid");
        path
    }
    else{path_file}
}