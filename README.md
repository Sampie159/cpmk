# Sam's C/C++ Project MaKer
## Generate a barebones C/C++ project structure including a basic Makefile
### Usage: cpmk \<language\> \<project-name\>

#### Example C:
```sh
cpmk c c-project
```

#### Example C++:
```sh
cpmk cpp cpp-project
```
or
```sh
cpmk cc cpp-project
```

### Output
```
c-project/
|
└───bin/
└───obj/
└───src/
└───Makefile
```

# Install
### Requires Cargo/Rust
Simply run
```sh
cargo install cpmk
```

# Uninstall
Simply run
```sh
cargo uninstall cpmk
```
# Built in Rust btw
