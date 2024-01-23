mod phext;

fn main() {
  println!("hello-phext v0.0.2");
  let test: phext::Coordinate = phext::to_coordinate("1.1.1/1.1.1/1.1.2");
  let address = test.to_string();
  println!("phext address: {address}");

  let expect1 = "Hello World";
  let expect2 = "Scroll #2 -- this text will be selected";
  let expect3 = "Scroll #3 - this text will be ignored";
  let sample = format!("{expect1}{expect2}{expect3}");
  let text = phext::fetch(&sample, test);
  println!("text at {test}: {text}.");

  let coord1 = phext::to_coordinate("1.1.1/1.1.1/1.1.1");
  let coord2 = phext::to_coordinate("1.1.1/1.1.1/1.1.2");
  let coord3 = phext::to_coordinate("1.1.1/1.1.1/1.1.3");

  let c1 = coord1.x.scroll;
  let c2 = coord2.x.scroll;
  let c3 = coord3.x.scroll;
  println!("Scrolls: {c1}, {c2}, {c3}.");

  let text1 = phext::fetch(&sample, coord1);
  let text2 = phext::fetch(&sample, coord2);
  let text3 = phext::fetch(&sample, coord3);
  let m1 = expect1 == text1;
  let m2 = expect2 == text2;
  let m3 = expect3 == text3;

  // Verify basic scrolls
  println!("Scroll Test 1: {m1} with '{text1}' vs '{expect3}'");
  println!("Scroll Test 2: {m2} with '{text2}' vs '{expect3}'");
  println!("Scroll Test 3: {m3} with '{text3}' vs '{expect3}'");
}