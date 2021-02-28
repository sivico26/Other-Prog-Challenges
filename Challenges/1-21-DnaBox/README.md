# __Kmer Counting Challenge__

### The Challenge

This challenge proposed to find the most frequent 9-mer of several genomes of [_Pseudomonas Syringae_](https://www.ncbi.nlm.nih.gov/assembly/?term=Pseudomonas+syringae) (from the previous link, hit on download assemblies and download them as fasta, this should retrieve 30 fasta files).

Why 9-mer? Well It is supposed that ORIs are usually of this size, so the most frequent kmer of these is likely a candidate to be one.

Still we can just abstract the challenge to be a general kmer counter.

### The Solution

We did the heavy-work of this challenge with [Rust](https://www.rust-lang.org/). Our implementation resulted in the crate `./dna_box`. 

To build the package (with optimizations):

```bash
cargo build --release
```

This should take just a couple of minutes and result (with 2 ignorable warnings) in the executable `./dna_box/target/release/dna_box`

`dna_box` reads a fasta file from `StdIn` and counts the 9-mers of the present sequences. The frequencies are put in a new folder `./Results_freqs` in which there is a new file for each **sequence** in the fasta provided. 

The computation can be easily calculated like this:

```bash
cat ncbi-genomes-2021-02-23/* | dna_box/target/release/dna_box
```

The results can then by gathered for easier handling:

```bash
paste Results_freqs/* > tot_results.tsv
```

Finally, we also created a histogram plotter in python (`kmer_frequencies_plotter.py`) to visualize the results and find out the most frequent Kmer.

In the end, it turned out that the most frequent kmer was `CAGCGCCAG` with 14202 occurrences. 

### The Design

We have not talked of the most interesting part of solving a challenge... How do we came up with a solution in a first place? After all, there can be many ways to solve the same problem. 

For the Kmer-counting crate (dna-box), besides [`rust-bio`](https://github.com/rust-bio/rust-bio), which we used to read the fasta file, we only used the Rust standard library. We ended up creating a couple of files:

- **main.rs**: The main function of the crate. Its role is to 
  1. Create the folder where to put the results. 
  2. Receive the input from StdIn (the fasta file contents).
  3. Pass the sequences to the kmer functions to do the computing.
  4. Write the vectors of frequencies in separate files.  
- **kmer_functions.rs**: Here are the important functions that do most of the work, specifically:
  - **kmer2idx**: Takes a kmer and generates a unique index in the frequencies vector (See rationale below).
  - **freq_kmers**: Takes the sequence provided by the main function and go over it counting the kmers and populating the frequencies vector.

There is also an **idx2kmer** function (that does the opposite of **kmer2idx**) but we actually did not use it in dna-box (this is one of the warnings you will get from the compiler). We did use it in the python script though.

After running `dna_box` and collecting the results, `kmer_frequencies_plotter.py` comes to play. Simple enough, it takes the resulting matrix using `numpy`, computes the aggregated kmer frequencies across samples (sounds complex, but it is just a sum ;) ), infers the most frequent kmer, and plot an histogram of the frequencies.

### The Rationale

Having introduced the structure of the functions we can explain why it is done in this way.

As the final result is a list of kmers with their respective frequencies (a 2 column file for instance), we need to eventually come up with vector, list or array that will store these frequencies. In languages like Python, these association of kmer-frequency would suit a dictionary pretty well, and indeed Rust has a analog structure (A [Hash-Map](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)). Unfortunately, this is not very efficient because to use this structures we need to compute a hash every time we want to retrieve the position of kmer and update its frequency. Generally, hashing is fast, but if we are going to do an operation millions of times (there are a few millions of kmers in each bacterial genome), we want it to be as cheap as possible.

Fortunately, there is a way of doing a meta-association without a Hash-map. Dealing with kmers presents an advantage in this case: They can be sorted lexicographically, and the result is deterministic (always the same). Thus, we can assign a base (4 in this case) to represent the kmer alphabet (`ACGT`)  with numbers (`0123`) Resulting in a code: ` A:0, C:1, G:2, T:3`. Thus a kmer such as `TCAGT` can be encoded as `31023`. Why would we want to do this? Well because a number such as `31023` in base 4 can be converted to another number in base 10 (In this case `843`). As the different kmers are unique **and** can be sorted, every number is associated with **one and only one** number. This allows us to establish these numbers as the index of a linear structure such a list or an array without depending on a Hash! (Actually, we can sort of say we are making our own hash function, but the trick is this one is faster to compute).

That might got a bit confusing, to illustrate we are making an system that gives to every kmer an unique <s>position</s> index  in an array/list/vector. So for example, let us take a kmer of size <img src="https://render.githubusercontent.com/render/math?math=k=4">, There a total of <img src="https://render.githubusercontent.com/render/math?math=base^{k} = 4^4 = 256"> possible kmers. The encoding would results in something like this:

| Kmer                                                         | index                                                        |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| AAAA                                                         | 0                                                            |
| AAAC                                                         | 1                                                            |
| AAAG                                                         | 2                                                            |
| AAAT                                                         | 3                                                            |
| AACA                                                         | 4                                                            |
| <img src="https://render.githubusercontent.com/render/math?math=\vdots"> | <img src="https://render.githubusercontent.com/render/math?math=\vdots"> |
| TTTT                                                         | 255                                                          |

**After all this errant, the rationale:** The first step is to create an array (that will store the frequencies) where all the kmers can be stored. This is done with the formula introduced above. `kmer2idx` does the kmer-index encoding: given a kmer it returns the index on the array where the kmer belongs. `freq_kmers` goes over the sequence that the `main` function provided making slices of size k (kmers) and calls to `kmer2idx` to know which index of the array should be updated. 

The above is done for every sequence found in the fasta and the results are stored in a separate file for each sequence.