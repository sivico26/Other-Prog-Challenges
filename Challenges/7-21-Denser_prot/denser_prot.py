#!/usr/bin/env python
from dataclasses import dataclass
from collections import defaultdict
import sys

"Define a dataclass to have the header information associated with the sequence"
@dataclass
class ProtSeq:
    name: str
    seq: str

mol_weights = defaultdict(lambda:136.9) ## In case we find weird characters 
mol_weights.update({
"A":89.1,"R":174.2,"N":132.1,"D":133.1,
"C":121.2,"E":147.1,"Q":146.2,"G":75.1,
"H":155.2,"I":131.2,"L":131.2,"K":146.2,
"M":149.2,"F":165.2,"P":115.1,"S":105.1,
"T":119.1,"W":204.2,"Y":181.2,"V":117.1,
"U":168.065
})        
        
def fasta_parser(file):
    """Fasta files parser. It takes the path to a fasta file, opens it and reads 
    it until exhaustation. Each entry is parsed with the ProtSeq Class. A list of
    ProtSeq instances for each entry in the fasta is returned."""
    with open(file) as fasta:
        prots = []
        header = None
        for line in fasta:
            if line.startswith(">"):
                #if not header:
                #    pass
                #else:   
                try: ## This could be better, right?
                    prots.append(ProtSeq(header, seq))
                except NameError:
                    pass
                seq = ""
                header = line.strip()
            else:
                seq += line.strip()
        else:
            prots.append(ProtSeq(header, seq))
        
        return prots
    
def density(prot):
    """Calculate the density of a protein. Sums the molecular weight of each
    aminoacid of the protein and divides it by the length of the protein."""
    return sum(mol_weights[aa] for aa in prot.seq) / len(prot.seq)

def write_winner(prot, density, out = sys.stdout):
    """Take the winner protein a write it to stdout in fasta format."""
    print(prot.name,f"[density={round(density, 3)}]", file = out) ## Header
    for i in range(0, len(prot.seq), 60):                         ## Sequence
        print(prot.seq[i:i+60], file = out)
    
def denser_prot(file):
    """Main function. Receive the fasta file as input and print the most dense to stdout."""
    prots = fasta_parser(file)
    densities = [density(prot) for prot in prots]
    idx = densities.index(max(densities))
    write_winner(prots[idx], densities[idx])
    
if __name__ == "__main__":
    denser_prot(sys.argv[1])
