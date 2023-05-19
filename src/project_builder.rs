use std::{
    env::current_dir,
    fs::{create_dir, write},
};

/// Generate the directories for the project.
fn create_directories(project_name: &String) -> String {
    let current_directory = current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let project_path = current_directory + "/" + project_name;
    match create_dir(&project_path) {
        Err(x) => println!("Error creating project directory: {x}"),
        Ok(_) => (),
    }

    let binaries_dir = project_path.clone() + "/bin";
    match create_dir(binaries_dir) {
        Err(x) => println!("Error creating project directory: {x}"),
        Ok(_) => (),
    }

    let objects_dir = project_path.clone() + "/obj";
    match create_dir(objects_dir) {
        Err(x) => println!("Error creating project directory: {x}"),
        Ok(_) => (),
    }

    let sources_dir = project_path.clone() + "/src";
    match create_dir(sources_dir) {
        Err(x) => println!("Error creating project directory: {x}"),
        Ok(_) => (),
    }

    project_path
}

/// Generate the files for the given language.
fn setup_files(language: &String, project_name: &String, project_path: String) {
    let compiler;

    if language.eq("c") {
        compiler = "gcc";
    } else {
        compiler = "g++";
    }

    let makefile_path = project_path.clone() + "/Makefile";
    let makefile_contents = format!(
        "CFLAGS=-Wall -Wextra -g\n\
CC={compiler}\n\
\n\
SRCDIR=src\n\
SRCS=${{wildcard $(SRCDIR)/*.{language}}}\n\
\n\
OBJDIR=obj\n\
OBJS=${{patsubst $(SRCDIR)/%.{language}, $(OBJDIR)/%.o, $(SRCS)}}\n\
\n\
BINDIR=bin\n\
\n\
all: $(BINDIR)/{project_name}\n\
\n\
release: CFLAGS=-Wall -Wextra -DNDEBUG -pipe -march=native -O2\n\
release: $(BINDIR)/{project_name}\n\
\n\
run: $(BINDIR)/{project_name}\n\
    \t$(BINDIR)/{project_name}\n\
\n\
$(OBJDIR)/%.o: $(SRCDIR)/%.{language}\n\
    \t$(CC) $(CFLAGS) -c -o $@ $^\n\
\n\
$(BINDIR)/{project_name}: $(OBJS)\n\
    \t$(CC) $(CFLAGS) -o $@ $^\n\
\n\
clean:\n\
    \trm -rf $(OBJDIR)/*\n\
    \trm $(BINDIR)/{project_name}"
    );
    match write(makefile_path, makefile_contents) {
        Err(x) => println!("Error creating Makefile: {x}"),
        Ok(_) => (),
    }

    let main_path = project_path + format!("/src/main.{language}").as_str();

    let main_content;

    if language.eq("c") {
        main_content = "#include <stdio.h>

int main() {
    printf(\"Hello World!\\n\");

    return 0;
}";
    } else {
        main_content = "#include <iostream>

int main() {
    std::cout << \"Hello World!\\n\";

    return 0;
}";
    }

    match write(main_path, main_content) {
        Err(x) => println!("Error creating main file: {x}"),
        Ok(_) => (),
    }
}

/// Checks if the given language is valid.
fn is_valid_language(language: &String) -> bool {
    let languages = [String::from("c"), String::from("cc"), String::from("cpp")];
    languages.contains(&language)
}

/// Creates a new project with the given name and language.
pub fn setup_project(args: Vec<String>) {
    let language = &args[1];
    let project_name = &args[2];

    if is_valid_language(&language) {
        let project_path = create_directories(&project_name);
        setup_files(language, &project_name, project_path);
        println!("Project {project_name} initiated succesfully!");
    } else {
        println!("Language not supported, use \"c\", \"cc\" or \"cpp\"");
    }
}
