# How to build
1. Execute the following command anywhere in the project directory:  
   ```shell
   cargo build [--release]
   ```
2. A new folder named '*target*' has been created in the root of the project directory.

# How to run
1. Build the application.
2. Change you directory to: `<project root>/target/<build>/`

   Note that `<build>` is either `debug` or `release` depending on whether the `--release` flag was provided during the build step.
3. Start a terminal to execute the '*parity_game_solver*' executable.