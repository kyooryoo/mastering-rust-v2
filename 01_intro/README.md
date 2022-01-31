## Introduction

### Modules
For managing code in small blocks:  
Java treats each .java file as a class.  
C++ has head file and *include* clause.  

Rust uses the concept of modules:  
Each Rust program has a root module  
For executable, it usually is *main.rs*.  
For library, it usually is *lib.rs*.  

Modules are delcared in other modules.  
Or can be organized as a file or folder.  

Use keyword *mod* so complier can know it.  
Import it with keyword *use* in root module.  

Items in module are private by default.
Use keyword *pub* to expose themm to external.