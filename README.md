# Hybrid-DataStructure
A hybrid data structure to store Strings, expected to be used for Heuristics for Bitcoin Forensics.<br />
Currently focusing on Heuristic H1, wich checks for same input addresses in different transactions, it makes heavy use of intersection and union functions.<br />
Eventually this project will become a external crate at crates.io to be used openly.

## Details about the project

It is a hybrid data structure that contains a BtreeSet and a BloomFilter.
The BloomFilter's goal is to optimize "contains" methods without compromising insertions.<br />
The current code is still on the optimization process.<br />

## Future updates:
To further optimize unions I am working on a way to store a list of the hybrid data structure, so in the case of an union of two big sets, a push
to the end of the list can solve it. My current problem with this solution is to make iterators work properly.<br />
I also want to test other Tree sets so the user can choose the best solution for the case.