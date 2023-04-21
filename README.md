# Hybrid-DataStructure
A hybrid data structure to store Strings, expected to be used for Heuristics for Bitcoin Forensics.<br />
Currently focusing on Heuristic H1, wich checks for same input addresses in different transactions, it makes heavy use of intersection and union functions.<br />
Eventually this project will become a external crate at crates.io to be used openly.<br />
This project is also part of my scientific initiation at UFU (Federal university of Uberlândia - Brazil).<br />

## Benchmarks and tests
"cargo bench" for benchmarks and "cargo test" for tests.

## Details about the project

It is a hybrid data structure that contains a BtreeSet and a BloomFilter.
The BloomFilter's goal is to optimize "contains" methods without compromising insertions.<br />
The current code is still on the optimization process but is relatively fast.<br />

## Union functions
Last version's union function just pushed the other data structure to the end of the list, this thing was causing some mutability and lifetime problems, it is possible to circle around it but for this new version I've changed the insertion function so it checks for duplicates and union basically inserts other into self. I did this to avoid this
mutability and lifetimes issues until I discover how to properly deal with it.<br />

## For the future updates:
* Make a better way of concatenating the iterators, because of borrowing issues, otherwise from my testings it works.<br />
* Make a function that returns iterators with the intersection itself, for the heuristics just a boolean value is enough, but I want it to make this structure more useful for other cases.<br />
* I also want to test other Tree sets so the user can choose the best solution for the case.


## Things I have already tried
* I tried to make it generic, not only for strings, but I don't know why it lost 40% performance since typing should be dealt with in compile time, not run time.<br />
* Also tried to make only one TreeSet and a list of BloomFilters, but it ended up consuming so much memory that it crashes when I test it with millions of strings.
The goal of this was to deal with duplicates from unions.<br />
* Currently using the same seed for all bloomfilters, with the hope to improve performance but haven't seen something relevant.