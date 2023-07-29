use std::{
    env::current_dir,
    fs::{create_dir, write},
};

use clap::Parser;

/// Struct that holds the projects information.
#[derive(Parser, Debug)]
#[command(name = "CPMK", version, about, author)]
pub struct Cpmk {
    /// The language of the new project
    #[clap(short, long)]
    language: String,

    /// The name of the new project
    #[clap(short, long)]
    project_name: String,
}

impl Cpmk {
    /// Setup the project
    pub fn setup_project(&self) {
        if !self.has_valid_language() {
            println!("Language not supported, use \"c\", \"cc\" or \"cpp\"");
            return;
        }

        let mut project_path = current_dir()
            .expect("Directory should be available")
            .into_os_string()
            .into_string()
            .expect("Directory should be a string");
        project_path = project_path + "/" + &self.project_name;

        self.create_directories(&project_path);
        self.create_files(&project_path);

        println!("Project {} created successfully.", &self.project_name);
    }

    /// Checks if the given language is valid.
    fn has_valid_language(&self) -> bool {
        const LANGUAGES: [&str; 3] = ["c", "cc", "cpp"];
        LANGUAGES.contains(&self.language.as_str())
    }

    /// Create the directories for the project
    fn create_directories(&self, project_path: &str) {
        create_dir(&project_path).expect("Directory should be created");

        let src_path = project_path.to_owned() + "/src";
        create_dir(src_path).expect("Directory should be created");
    }

    /// Create the files for the project
    fn create_files(&self, project_path: &str) {
        let main_file = project_path.to_owned() + "/src/main." + &self.language;
        let cmake_file = project_path.to_owned() + "/CMakeLists.txt";
        let cmake_src_file = project_path.to_owned() + "/src/CMakeLists.txt";

        let main_file_content = if &self.language == "c" {
            "#include <stdio.h>

int main(void) {
  printf(\"Hello World!\\n\");

  return 0;
}"
        } else {
            "#include <iostream>

int main(void) {
  std::cout << \"Hello World!\\n\";

  return 0;
}"
        };

        let cmake_extra = if &self.language == "c" {
            "set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED True)
set(CMAKE_C_FLAGS \"${CMAKE_C_FLAGS} -Wall -Wextra -Wpedantic\")\n\n"
        } else {
            ""
        };

        let cmake_file_content = format!(
            "cmake_minimum_required(VERSION 3.10)

project({})

{}set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_FLAGS ${{CMAKE_CXX_FLAGS}} -Wall -Wextra -Wpedantic)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${{CMAKE_BINARY_DIR}})

add_subdirectory(src)\n",
            self.project_name, cmake_extra
        );

        let cmake_src_content = format!(
            "cmake_minimum_required(VERSION 3.10)

add_executable(
  {}
  main.{}
)\n",
            &self.project_name, &self.language
        );

        write(main_file, main_file_content).expect("File should be created");
        write(cmake_file, cmake_file_content).expect("File should be created");
        write(cmake_src_file, cmake_src_content).expect("File should be created");
    }
}
