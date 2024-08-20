use std::env;



fn main() {
    let args :Vec<String > = env::args().collect();
    if args[1] == "init"  {

    } else if args[1] == "generate" {
        println!("Generation...");
    }
}
