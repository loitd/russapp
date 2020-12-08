@REM Create a new project: cargo new
@REM Create a new project in an existing directory: cargo init
@REM Build the project: cargo build
@REM Run the project: cargo run
@REM Update project dependencies: cargo update
@REM Run tests: cargo test
@REM Run benchmarks: cargo bench
@REM Generate the project documentation via rustdoc: cargo doc
@REM Analyze the project to see it has any errors, without building it: cargo check
docker build -t russapp . 
docker run -it --rm --name running-russapp russapp