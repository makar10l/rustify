use std::io::{self, Write, ErrorKind, Error};
pub fn get_music_path() -> Result<String, Error>{
    let username = std::env::var("USER").expect("cannot identificate you");
    let path_file = 
    match std::fs::read_to_string(
        format!("/home/{}/.rustify/data/path_music",username)
    ){
        Err(err) => {
            match err.kind(){
                ErrorKind::NotFound => {
                    std::fs::File::create(format!("/home/{}/.rustify/data/path_music",username)).expect("IDK");
                    String::from("")
                },
                _ => return Err(err),
            }
        },
        Ok(result) => result, 
    };

    if path_file.is_empty(){
        println!("
            Your path to music is empty, please write ABSOLUTE path to your music <3:
        ");
        let mut path = String::new();
        io::stdin().read_line(&mut path).expect("OS error");
        path = path.trim().to_string();
        write!(std::fs::File::create(
            format!("/home/{}/.rustify/data/path_music",username)
        ).unwrap(), "{}", path).expect("writing path error");
        Ok(path)
    }
    else{Ok(path_file)}
}