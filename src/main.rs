mod phext;

fn main() {
  println!("hello-phext v0.0.1");
  let test: phext::Coordinate = phext::to_coordinate("1.1.1/1.1.1/1.1.1");
  let address = test.to_string();
  println!("phext address: {address}");

  let sample = "Hello World
Scroll #2";
  let text = phext::fetch(sample, test);
  println!("text at {test}: {text}.");
}