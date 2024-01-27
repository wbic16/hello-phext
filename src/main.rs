mod phext;
mod phext_test;

fn main() {
    println!("hello-phext v0.0.5");

    let buffer = "Quick Example\x17Second Scroll\x18Second Section\x19Section Chapter";
    let scroll1 = phext::locate(buffer, "1.1.1/1.1.1/1.1.1");
    let scroll2 = phext::locate(buffer, "1.1.1/1.1.1/1.1.2");
    let scroll3 = phext::locate(buffer, "1.1.1/1.1.1/1.2.1");
    let scroll4 = phext::locate(buffer, "1.1.1/1.1.1/2.1.1");

    println!("The Reference Phext has 4 scrolls, listed below:");
    println!("* {scroll1}");
    println!("* {scroll2}");
    println!("* {scroll3}");
    println!("* {scroll4}");
}