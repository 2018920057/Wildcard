use std::io;

fn main() {
  let mut a: String = String::new();
  println!("매치할 패턴:");
  io::stdin().read_line(&mut a).expect("error");
  let mut b: String = String::new();
  println!("매치할 문자:");
  io::stdin().read_line(&mut b).expect("error");
  println!("{:?}",matchString(&a,&b));
}

fn matchString(a: &String,b: &String) -> bool{
  //TODO
  false
}