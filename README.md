# senzu
A vanity address generator that uses your xpub to find an address. 

It works by derivating your xpub with different paths until an address with a desired prefix is found.
Starting with m/0 it tries every possible path in a breadth first way, 
so once all combinations in a derivation depth is exhausted it'll add 
another index (in this case m/0/0) and repeats the whole process until a fitting address is found.
Derivation of the rightmost index can be out of order because the task is split into multiple threads.


## Installation
...


## Example usage
...


## Useful resources
...