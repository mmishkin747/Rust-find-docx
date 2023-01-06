# rust-find-docx
This is CLI program for find and open file docx

## Usage

for exaples:

1. open docx file
```sh
rfd tests/input/hello.docx 
```
2. Find pattern "Test" in file 
```sh
rfd test/input/hello.docx -p Test
```
3.Find pattern "Test" in file with insensitive case
```sh
rfd test/input/hello.docx -p test -i
```
4. Find all docx file in directory
```sh
rfd test/input/
```

## Running the tests

There are six tests.

```sh
cargo test
```

