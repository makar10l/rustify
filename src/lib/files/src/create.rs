pub fn create_data(){
    let user = std::env::var("USER").expect("Error with getting username");
    std::fs::create_dir(format!("/home/{}/.rustify", user)).expect("Error: Creating dir");
    std::fs::create_dir(format!("/home/{}/.rustify/data", user)).expect("Error: Creating dir");
    std::fs::create_dir(format!("/home/{}/.rustify/bin", user)).expect("Error: Creating dir");
    std::fs::copy("/bin/.main_rustify", format!("/home/{}/.rustify/bin/rustify", user)).expect("Error: Copying bin");
}
