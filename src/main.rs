use project_builder::setup_project;

mod project_builder;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    if args.len() != 3 {
        println!("Incorrect usage, please run something like:\ncpmk <language> <project-name>");
    } else {
        setup_project(args);
    }
}
