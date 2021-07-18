#!/usr/bin/env python
"""To use this script, either provide a the path to a fastq file and a quality
value (2 arguments, output will be written to stdout), or... 

python trimming.py reads.fastq 20

""" ## Finish this
from dataclasses import dataclass
from collections import namedtuple
import sys

"Define a dataclass to have the header information associated with the sequence"
@dataclass
class Read:
    name: str
    seq: str
    qual: list

def fastq_parser(file):
    """Fastq files parser. It takes the path to a fasta file, opens it and reads 
    it until exhaustation. Each entry is parsed with the ReadSeq Class. A list of
    ReadSeq instances for each entry in the fastq is returned."""
    with open(file) as fastq:
        reads = []
        header = None
        for line in fastq:
            if line.startswith("@"):
                try:
                    qual = list(map(lambda x: ord(x) - 33, qual))
                    reads.append(Read(header, seq, qual))
                except NameError:
                    pass
                qual_zone = False
                seq, qual = "", ""
                header = line.strip()
            elif line.strip() == "+":
                qual_zone = True
            elif qual_zone:
                qual += line.strip()
            else:
                seq += line.strip()
        
        qual = list(map(lambda x: ord(x) - 33, qual))
        reads.append(Read(header, seq, qual))
        return reads
    
def trim_pos(read, quality, w):
    """Find positions for trimming based on que quality profile of a read
    and a given quality and window size. Returns start and end (trimming positions)."""
    middle = round(len(read.qual) / 2)
    start = end = None
    
    for i in range(middle, w - 1, -1):
        window = read.qual[i-w : i]
        mean_qual = sum(window) / w    
        if mean_qual < quality:
            for j in range(i, i - w, -1):
                if read.qual[j] < quality:
                    start = j
                    break
        elif start:
            break
        
    for i in range(middle, len(read.qual) - w):
        window = read.qual[i: i+w]
        mean_qual = sum(window) / w    
        if mean_qual < quality:
            for j in range(i, i + w):
                if read.qual[j] < quality:
                    end = j
                    break
        elif end:
            break
            
    return start, end

def write_fastq(reads, out = sys.stdout):
    """Format a collection of reads as .fastq and write to a file.
    Default writes to stdout."""
    if out is not sys.stdout: 
        out = open(out, "w")
        
    for read in reads:
        print(read.name, file = out)
        for i in range(0, len(read.seq), 60):
            print(read.seq[i:i+60], file = out)
            
        print("+", file = out)
        qual = "".join(map(lambda x: chr(x + 33) , read.qual))
        for i in range(0, len(qual), 60):
            print(qual[i:i+60], file = out)
    out.close()

def trimming(file, quality, outfile = sys.stdout, window = 5):
    """Main function. Receives reads in fastq format and a quality value in phred scale
    and trims them so that most of the bases of the read are above this quality. Output
    is written in fastq format to the specified file or, by default, to stdout."""
    reads = fastq_parser(file)
    positions = [trim_pos(read, quality, window) for read in reads]
    for (start, end), read in zip(positions, reads):
        read.seq = read.seq[start:end]
        read.qual = read.qual[start:end]
    write_fastq(reads, outfile)
    
if __name__ == "__main__":
    if len(args := sys.argv) < 2:
        print("Expected at least 2 arguments. Ensure to provide an fastq file and a phred quality value.")
    elif len(args) == 2 or len(args) == 4:
        trimming(*args)
    elif len(args) == 3:
        
