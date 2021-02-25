use std::env::current_dir;

#[test]
fn m(){
    let current_path = current_dir().unwrap();
    println!("{:?}",current_path);
}