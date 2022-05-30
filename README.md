# senzu
A vanity address generator that uses your xpub to find addresses. 

It works by deriving your xpub with different paths until an address with a desired prefix is found.
Starting with m/0 it tries every possible path in a breadth first way, 
so once every combination in a derivation depth is exhausted it'll add 
another index (in this case m/0/0) and repeats the whole process until a matching address is found.
Derivation of the rightmost index can be out of order because the task is split into multiple threads.

## Installation
```
$ cargo install senzu
```

## Usage
```
senzu 0.1.0
Vanity address generator searching addresses derived from your xpub

USAGE:
    senzu --prefix <prefix>... --xpub <xpub>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --prefix <prefix>...    Prefixes of your desired vanity address
        --xpub <xpub>           xpub to derive addresses from
```

## Example usage
```
$ senzu --prefix bc1qfree bc1qkek  --xpub xpub661MyMwAqRbcEjCyfkRFXufSZxJgPvWhSo3Nv4L4reXA2WT9hQUGH4eMfzmiPPbBtfSaCEyFHsfaZvfoCQBX7fggRVEtt5n6DBAjcGjMmQ5

Found address bc1qkekvh074w2gwz5jrtl07rcg0es0gvj6z0r5evd at path m/81253
```
