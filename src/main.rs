mod phext;
mod phext_test;
use std::fs;
use std::env;

fn main() {
    println!("hello-phext v0.0.6");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        println!("\navailable commands:");
        println!(" - pack <archive>: packs the given archive as a phext");
        println!(" - unpack <phext>: unpacks the given phext into your local directory");        
        println!(" - help: print an example phext");
        return;
    }

    let command = &args[1];    

    if command == "pack"
    {
        if args.len() < 3
        {
            println!("Missing file parameter for pack.");
            return;
        }
        let file = &args[2];
        run_pack(file);
        return;
    }

    if command == "unpack"
    {
        if args.len() < 3
        {
            println!("Missing file parameter for unpack.");
            return;
        }
        let file = &args[2];
        run_unpack(file);
        return;
    }
    
    if command == "help"
    {
        run_example();
    }
}

fn ignore_path(path: String, file: &str) -> bool {
    return path.ends_with(".git") ||
           path.ends_with("Cargo.lock") ||
           path.ends_with(".gitignore") ||
           path.ends_with(file);
}

fn run_pack(file: &str)
{
    println!("Packing local files into {file}...");
    let mut coord = phext::to_coordinate("1.1.1/1.1.1/1.1.1");

    let paths = fs::read_dir("./").unwrap();
    let mut output = String::new();

    for ith in paths
    {
        let path = ith.unwrap().path();
        let value = path.clone().into_os_string().into_string().unwrap();
        if !ignore_path(value.clone(), file) && path.is_file()
        {
            println!("{coord}: {value}");
            let data = fs::read_to_string(value).expect("unexpected read error");
            output.push_str(data.as_str());
            output.push(phext::SCROLL_BREAK);
            coord.scroll_break();
        }
    }

    fs::write(file, output.clone()).expect("Error writing output");
}

fn run_unpack(file: &str) {
    println!("Pretending to extract {file} to local directory...(not yet implemented)");
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