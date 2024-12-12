# Overview


This program employs Rust to design an insulin-dose-calculator program. Since insulin dose calculators are usually embedded into medical devices, so Rust is an exceptional choice because it ensures the memory safety during runtime based on its features such as ownership, multithreaded processing, error handling...

Firstly, It takes 5 input from users: current glucose leevel, target glucose level, insulin sensitivity factor, carbohydrate intake and carb-to-insulin ratio. object-oriented techniques such as struct and implementation are used.  Next, it calculates correction dose, carb dose and total dose needed. Mutable and immutable variables, match expressions, for loops, if conditions and functions using reference are used. Finally, it displays dose history. Data structure of Vector is used. 

I have a family member who has type one diabetes and uses insulin dose calculoator everyday so I am very interested how it works. I would like to develop a handy software to benefit people who are in need. 

[Software Demo Video](https://youtu.be/gMppzaAV3Ac)

# Development Environment

- Rust with std::io library 
- Visual studio code with Rust Analyzer

# Useful Websites


- [Install Rust](https://www.rust-lang.org/tools/install)
- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [The Rust Programming Language](https://doc.rust-lang.org/book/index.html)
- [Diabetes Education](https://diabetes.org/tools-resources/tests-calculators)

# Future Work

- Save dose history to a file so users can review past sessions.
- Develop a desktop GUI using Rust's egui or Tauri to provide a more user-friendly interface.
- Encrypt user inputs and stored files to protect sensitive health data.
