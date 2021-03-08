# rs-ncbigap
![](https://github.com/Ubsefor/rs-ncbigap/workflows/Rust/badge.svg)

A small program on Rust to count gaps and unknown genomes in NCBI’s fasta sequences

# Building:

You need Rust's Cargo to build the project. 

Follow the official installation instructions to get it: [guide](https://www.rust-lang.org/tools/install)


On Unix-like systems with rust available: 
`git clone https://github.com/Ubsefor/rs-ncbigap ; cd rs-ncbigap ; cargo build --release`

# Running:

From the project's root directory:

`cargo run --release -- args` where args can be:

`-h`: prints usage message

`-v`: prints version

`<filepath>` – place a path to the file here, for the given examples you can just write the name of the file

Note, that the program accepts only files, containing raw genetic code (see [example](NC_000001.11 Homo sapiens chromosome 1, GRCh38.p13 Primary Assembly.fasta) to get the idea), so if you download genomes from  [NCBI](https://www.ncbi.nlm.nih.gov), use FASTA format and remove the header message!

# Testing:

You can also run `cargo test` mainly to see if your rust installation is working properly.


