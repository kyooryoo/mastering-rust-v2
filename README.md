# Mastering Rust V2

## Tips and Tricks
Here are some general and common tips and tricks.  

### Mute warings
Mute various warnings about unused things in debug:  
* add *#[allow(dead_code)]* above each line that causes warning  
* add *#![allow(dead_code)]* as the first line in source file
Using *cfg* can mute warnings in debug mode only:  
`#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]`
Use the above code as the first line will mute once for all.  
However, `cargo build --release` will still generate warnings.  

  