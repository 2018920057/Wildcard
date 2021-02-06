use std::io;

fn main() {
  loop{
  let mut a: String = String::new();
  println!("매치할 패턴:");
  io::stdin().read_line(&mut a).expect("error");
  let mut b: String = String::new();
  println!("매치할 문자:");
  io::stdin().read_line(&mut b).expect("error");
  println!("{:?}",matchString(&a[..],&b[..]));
  }
}

fn matchString(a: &str,b: &str) -> bool{
  let alen: usize = a.len();
  let blen: usize = b.len();
  let mut i: usize = 0;
  //a[i]와 b[i]가 같거나 a[i]가 ?라면 인덱싱 진행
  while i<alen && i<blen && a.chars().nth(i).unwrap() != '*' && (a.chars().nth(i).unwrap() == '?' || a.chars().nth(i) == b.chars().nth(i)){
    i += 1;
  }
  //i가 패턴의 끝까지 도달하면 b의 끝까지 왔는지의 여부에 따라 같거나 다름
  if i==alen{
    return i==blen;
  }
  //패턴에 *이 나왔을 때 하나씩 스킵하며 반복
  if a.chars().nth(i).unwrap()=='*'{
    for skip in 0..blen-i{
      if matchString(&a[i+1..],&b[i+skip..]) {return true;}
    }
  }
  false
}