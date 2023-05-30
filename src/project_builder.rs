use std::{
    env::current_dir,
    fs::{create_dir, write},
};

/// Helper function to create a directory.
fn create_directory(directory: &str) {
    create_dir(directory).expect("Error creating project directory");
}

/// Generate the directories for the project.
fn create_directories(project_name: &str) -> String {
    let current_directory = current_dir()
        .expect("Current directory should exist")
        .into_os_string()
        .into_string()
        .expect("Current directory path should be a string");

    let project_path = current_directory + "/" + project_name;
    create_directory(&project_path);

    let binaries_dir = project_path.clone() + "/bin";
    create_directory(&binaries_dir);

    let objects_dir = project_path.clone() + "/obj";
    create_directory(&objects_dir);

    let sources_dir = project_path.clone() + "/src";
    create_directory(&sources_dir);

    project_path
}

/// Generate the files for the given language.
fn setup_files(language: &str, project_name: &str, project_path: String) {
    let compiler = if let "c" = language { "gcc" } else { "g++" };

    let makefile_path = project_path.to_owned() + "/Makefile";
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
    if let Err(x) = write(makefile_path, makefile_contents) {
        println!("Error creating Makefile: {x}")
    }

    let main_path = project_path.clone() + format!("/src/main.{language}").as_str();

    let main_content = if let "c" = language {
        "#include <stdio.h>

int main() {
    printf(\"Hello World!\\n\");

    return 0;
}"
    } else {
        "#include <iostream>

int main() {
    std::cout << \"Hello World!\\n\";

    return 0;
}"
    };

    if let Err(x) = write(main_path, main_content) {
        println!("Error creating main file: {x}")
    }

    let gitignore_path = project_path + "/.gitignore";
    let gitignore_content = "bin/\nobj/";

    if let Err(x) = write(gitignore_path, gitignore_content) {
        println!("Error creating .gitignore file: {x}")
    }
}

/// Checks if the given language is valid.
fn is_valid_language(language: &str) -> bool {
    let languages = ["c", "cc", "cpp"];
    languages.contains(&language)
}

/// Creates a new project with the given name and language.
pub fn setup_project(args: Vec<&str>) {
    let language = &args[1];
    let project_name = &args[2];

    if is_valid_language(language) {
        let project_path = create_directories(project_name);
        setup_files(language, project_name, project_path);
        println!("Project {project_name} initiated succesfully!");
    } else {
        println!("Language not supported, use \"c\", \"cc\" or \"cpp\"");
    }
}
