#[test]
fn test_move(){
    let s = "123".to_string();
    std::thread::spawn(move ||{
       println!("{}",&s);
    });
    // println!("{}",s);
}