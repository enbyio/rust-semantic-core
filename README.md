# Triple Store frontend for pg-triple-store written in rust

This is a relatively simple frontend for the [pg-triple-store](https://github.com/enbyio/pg-triple-store.git) rust library.

Current Functionality primarily includes:
- importing turtle and rdf/xml files
- running sparql Select and Construct Queries
- exporting the Construct Query Results as rdf/xml files

## Setup
1. Create a .env file in the project root with the following contents (insert your chosen password in the noted location)
```sh
DATABASE_URL=postgres://postgres:{yourpassword}@localhost:5432/rust_semantic_core
```
2. Run `POSTGRES_PASSWORD={yourpassword} ./start_podman.sh`. The password parameter is only required on the first startup, your password has to be the same as in the `.env` file to work.

3. `cargo run --release` and open `127.0.0.1:8080`

## Running normally
1. Running the first time after starting your system you need to run `./start_podman.sh` again.

3. `cargo run --release` (the release flag is important here because for non release builds the store will reset and load some testing data on each start)
