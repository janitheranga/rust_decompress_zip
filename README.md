# rust_file_compressor
A simple file zip file extractor written in Rust

## Run

#### Example use case -> `data_ext` folder for zipping and `data.zip` for unzipping.

### 1. `Zipping`
1. Add the `traget` to the root of the project. As an example use `data_ext` folder.
2. Run `cargo run <target> Z`. Here `<target>` is `data_ext` and `Z` is the command to perform `zipping`.
3. Program will output `data_ext.zip` as a new file.

### 2. `Unzipping`
1. Add the `target` to the root of the project. As an example use `data.zip`.
2. Run `cargo run <target> U`. Here `<target>` is `data.zip` and `U` is the command to perform `unzipping`.
3. Program will output `data` folder with its content.
