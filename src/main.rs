mod phext;

fn main() {
  println!("hello-phext v0.0.1");
  let test: phext::Coordinate = phext::to_coordinate("1.1.1/1.1.1/1.1.1");
  let address = phext::to_string(test);
  println!("phext address: {address}");
}