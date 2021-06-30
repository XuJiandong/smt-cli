# smt-cli
Command line tools for SMT: https://github.com/jjyr/sparse-merkle-tree

# inclusion
```text
smt-cli --include "0|1" 11 22
```
Generate a sparse merkel tree, with hash values of:
```text
[11, 0, 0, ...]
```
and 
```text
[22, 0, 0, ...]
```
and generate root and proof, which show the hash at index 0 and 1 are on tree.
It's used for white list.

The arguments in "--include" are indexes, delimited by |.
# non-inclusion
```text
smt-cli --exclude "11|22" 111 222
```
Generate a sparse merkel tree, with hash values of:
```text
[111, 0, 0, ...]
```
and
```text
[222, 0, 0, ...]
```
and generate root and proof, which show the hash:
```text
[11, 0, 0, ...]
```
and 
```text
[22, 0, 0, ...]
```
are not on tree. It's used for black list.

The arguments in "--exclude" are hashes, delimited by |.

# Hash format on command line
There are 2 places in command line to show hash:
1. --exclude
2. trailing arguments

There are 2 formats to represent it:
* hex format
  which is started with "0x", then followed by hex string. For example, 
   * 0x0
   * 0xFFEF
  
  Normally they are used as string in Rust or C.
     
* array format
  plain number, delimited by ",". For example, 
   * 100,200,300
   * 0
  
  Normally they are used as literal in Rust or C.
     
padding with zero if the length is smaller than 32.  
Use "--hex" to print hash in hex format: by default, it's in array format.
