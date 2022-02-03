## Cargo
Use Cargo for managing project and dependencies.  
When program grows, better to manage by modules.  

### How to use module
Use keyword *mod* to import an external module.  
(Internal module has *mod* in its declaration) 

Use keyword *use* to import elements in module.  
* use crate::foo::Bar;
* use self::foo::Bar;
* use super::foo::Bar;
Keyword *crate* imports from the root module.  
KW *self* imports from current module itslef.  
KW *super* to import item from parent module.  

Folder structure can be used for managing modules.  
Notice, there are two methods of implementation:  
1. create both *foo* folder and *foo.rs* file
2. create *foo* folder and within *mod.rs* file

### What Cargo can do
When project grows, refactor code into small units.  
Create documents about its struct and dependencies.  
Create and manage online repository of the project.  

Cargo uses *crates.io* for storing program library.  
Rust program is known as *crate* from three sources:  
* Local directory
* Online Git repo such as GitHub
* Other online crate service such as *crates.io*

### Cargo commmands
Get general help: cargo  
Get help of a specific command:cargo help new  
Create new project: cargo new <pj_name>  

Update all dependencies: cargo update
Update specified package: cargo update -p <crate_name>  

Version number format: major.minor.patch  
* major: big change or critical bug fixation
* minor: add new feature in compatible mode
* patch: fix bug, no new feature, back compatible

Check code for errors but do not compile: cargo check  
Create executable in *target/debug/*: cargo run  
Create release version executable: cargo run --release  

### Cargo Test
Lib is created with an out-of-the-box texting code:  
`$ cargo new mytest --lib`
`$ cd mytest && cargo test`

### Cargo Examples
Add some example code to demo how to use your library.  
Put example files in a folder called *examples*.  
Run example with: `$ cargo run --example <file_name>`  

### Cargo Workspace
Workspace manages large project with multiple libraries.  
*Cargo.lock* file and public dir is shared among them.  
Main program and libraries are located in the same dir.  

*Cargo.toml* in a workspace contains a list of libraries:  
`members = ["my_crate", "app"]`  
*Cargo.toml* in main program lists libs in dependencies:  
`my_crate = { path = "../my_crate"}`  
Check out the demo project *workspace_demo*  

### Other Tools
Run `cargo install <tool_name>` to install them:  
* cargo-watch: auto check code when update happens
Run `cargo watch` once to start monitoring  
* cargo-edit: auto add dependencies into *Cargo.toml*
Run `cargo add` `cargo rm` `cargo edit` `cargo upgrade`  
* cargo-deb: generate Debian Linux executable
* cargo-outdated: check outdated libraries

* clippy: check and fix code syntax errors
Install with `rustup component add clippy`  
Run with `cargo clippy`  

### About Cargo.toml
*package* section contains the following fields:  
* description
* license
* readme
* documentation
* keywords
* authors
* build
* edition

*package.metadata.settings* section contain custom data  
* You can put any key-value pair here  
* Rust will not check

*features*, *build-dependencies* *dependencies* work together
* library version number with prefix *^* means *above*
+ such as `syntex = "^0.58"`
* library link could be specified with GitHub repo URL
+ such as `time = { git = "URL", branch = "master" }`
* specified dependent library could be optional
+ such as `sqlite = { version = "2.5", optional = true }`
* preferred optional library could be defined in *feature*
+ such as `default = ["sqlite"]`

### Dev Environment
IntelliJ IDEA or Visual Studio Code are preferred options.  
Recommended plugin is *rls-vscode* search *rls* can find it.
Check other IDEs for RUST at [Are we (I)DE yet?](https://areweideyet.com/)  

### Demo Project
Create a program with a 3rd party library *image*  
1. Add *image* library to *Cargo.toml* dependency
```
[dependencies]
image = "0.19.0"
```
Then run `cargo build` to get it ready  
2. Modify *main.rs* to read command param as image path