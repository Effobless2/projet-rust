# projet-rust 🥑

## Development Team
 - Mégane DABIN
 - Maxime DEBOFFLE
 - Romain LENOIR
 
 ## Subject
 A Beautiful Rust Library for manipulating ppm files for generating images usable in any Programmation languages
 
 ### Features
  - Copying an Image
  - Grayscaling an Image
  - Reversing an Image
  - Returning 42
  
  
  ### How to use it ?
  - Compile the library using `cargo build --release`
  - Choose your favourite programming language
  - Use a library to import binary files
  - import the compiled library into your code
  - Use following functions
  
  #### Dummy()
  - Returns 42
  #### copy_ppm_file(from, to)
  - Creates a new file with the `to` name which contains the same image as the `from` ppm file
  #### generate_invert_ppm_file(from, to)
  - Creates a new file with the `to` name which contains the negative image from the `from` ppm file
  #### generate_grayscale_ppm_file(from, to)
  - Creates a new file with the `to` name which contains the grayscaled image from the `from` ppm file
  
  ### How to test
  Run `cargo test`
  
  ### How to run benchmarks
  Run `cargo bench`
  
  ### Documentation
  You can find it in `target/doc` folder
