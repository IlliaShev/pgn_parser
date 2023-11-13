! Brief Description: Include a brief description of the idea in both the Readme.md and Cargo.toml files. The Readme.md must contain a technical description of the parsing process, detailing what exactly is being parsed and how the results of the parsing will be used.
! Project Name: The project name must reflect the idea and be included in both the Readme.md and Cargo.toml files.
! Grammar Rules: Develop at least 4 grammar rules for the parser.
! Unit Tests: Implement unit tests for the parser; these can be placed in a separate file within the tests directory.
! Test Coverage: Ensure each grammar rule is covered by at least one unit test.
! Core Files: The project must include lib.rs and main.rs files.
! CLI: Create a command-line interface that includes commands to parse a file and to display help and credits information.
Error Handling: Implement proper error handling by using anyhow for tests and thiserror for the library.
cargo fmt should be used for code formatting.
cargo clippy should be used for linting to ensure code quality.
A makefile should include commands to launch the program and to run tests, format and clipply before committing as well other useful commands.
Completeness: The project must be completed fully.
Including a diagram or grammar in the Readme.md can facilitate understanding of the parsing logic.
Documentation: For each grammar rule, embed documentation in the code, ensuring that the documentation appears on docs.rs.
Publishing: Release the final version of the package on crates.io.