mod phext;
mod phext_test;
use std::env;

fn main() {
    println!("hello-phext v0.0.5");

    let args: Vec<String> = env::args().collect();
    if args.len() < 3
    {
        println!("\navailable commands:");
        println!(" - pack <archive>: packs the given archive as a phext");
        println!(" - unpack <phext>: unpacks the given phext into your local directory");        
        return;
    }

    let command = &args[1];
    let file = &args[2];

    if command == "pack"
    {
        run_pack();
    }

    if command == "unpack"
    {
        run_unpack();
    }
    
    run_example();
}

fn run_pack() {

}

fn run_unpack() {
    
}

fn run_example() {
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