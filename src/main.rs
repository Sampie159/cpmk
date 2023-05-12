use project_builder::setup_project;

mod project_builder;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Incorrect usage, please run something like:\ncpmk <language> <project-name>");
    } else {
        setup_project(args);
    }
}
