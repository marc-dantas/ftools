# ftools
File Tools

- ftools is a simple collection of tools for working with files:
    + Searching strings in files
    + Comparing files
    + Reading files
> In the future, ftools will has more functionalities.

## Getting started
Build ftools using cargo:
```console
$ cd /path/to/ftools
$ cargo build --release
```
Run ftools:
```console
$ ./target/release/ftools  # or .\target\release\ftools.exe on Windows
```
Usage:
```
Usage: ftools <COMMAND> <ARGS>...
Commands:
  g, grep <FILE>  <PATTERN> Find lines matching PATTERN
  c, cat  <FILE>            Print the contents of FILE
  d, diff <FILE1> <FILE2>   Compare FILE1 and FILE2
  h, help                   Print the help message
```
## Example
### Grep, Diff and Cat
```console
$ cd /path/to/ftools
$ cargo build --release
$ echo "This is a test" >> test.txt
$ echo "This is a test" >> test2.txt
... snip ...
$ ./target/release/ftools grep test.txt "test"
Command: Grep
Match: This is a test
Total match count: 1
$ ./target/release/ftools diff test.txt test2.txt
Command: Diff
Comparing files: test.txt, test2.txt
Files are identical
$ ./target/release/ftools cat test.txt
Command: Cat
This is a test
```