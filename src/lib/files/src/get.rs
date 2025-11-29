use std::io::{self, Write};
pub fn get_path() -> String{
    let path_file = std::fs::read_to_string("data/path-to-config.txt").expect("please, buy me brain"); 
    
    if path_file.is_empty(){
        println!("
            Your path to music is empty, please write ABSOLUTE path to your music <3:
        ");
        let mut path = String::new();
        io::stdin().read_line(&mut path).expect("OS error");
        path = path.trim().to_string();
        write!(std::fs::File::create("data/path-to-config.txt").unwrap(), "{}", path).expect("im stupid");
        path
    }
    else{path_file}
    
}