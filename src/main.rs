use cpmk::Cpmk;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let language = &args[1];
    let project_name = &args[2];

    let cpmk = Cpmk::new(language, project_name);

    cpmk.setup_project();
}
