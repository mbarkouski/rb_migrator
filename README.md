# rb_migrator
## Run
Set environment variable to binary file

```rb_migrator -c <path to config file with connection to ClickHouse> -m <path to file with migration>```

## Need

  Rust installed 
  https://www.rust-lang.org/ru/tools/install
## Build
cargo build --release

##Migrations in sql file
If you perform several operations, separate them with the ```#``` sign (see example in migrations/1.sql).
