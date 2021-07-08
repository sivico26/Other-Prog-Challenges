use std::cmp::min;
use std::io::{self, Read};
use std::collections::HashMap;
use std::str::FromStr;
struct ProtSeq {
    name: String,
    seq: String
}

fn parse_fasta() -> Vec<ProtSeq> {
    let mut prots: Vec<ProtSeq> = Vec::new();
    let mut header = String::new();
    let mut seq  = String::new();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);
    
    for line in buffer.split("\n") {
        if line.starts_with(">") {
            if header.len() > 0 {
                prots.push(ProtSeq{name:header, seq: seq});
            }
            seq = String::new();
            header = String::from_str(line).ok().unwrap();
        } else {
            seq += line;
        }
    } 
    prots.push(ProtSeq{name:header, seq:seq});
    return prots
}

fn density(prot:&ProtSeq) -> f64 {
    let mol_weights: HashMap<char, f64> = 
    [('A',89.1), ('R',174.2), ('N',132.1), ('D',133.1),
    ('C',121.2), ('E',147.1), ('Q',146.2), ('G',75.1),
    ('H',155.2), ('I',131.2), ('L',131.2), ('K',146.2),
    ('M',149.2), ('F',165.2), ('P',115.1), ('S',105.1),
    ('T',119.1), ('W',204.2), ('Y',181.2), ('V',117.1),
    ('U',136.9)]
    .iter().cloned().collect();
    let tot_sum:f64 = prot.seq.chars().map(|aa:char| -> &f64 {mol_weights.get(&aa).unwrap_or(&136.9)}).sum(); 
    tot_sum / prot.seq.len() as f64
}

fn write_winner(prot:&ProtSeq, density:f64) {
    println!("{} [density={}]", prot.name, density);
    for i in (0..prot.seq.len()).step_by(60) {
        let end = min(i+60, prot.seq.len());
        println!("{}", &prot.seq[i..end]);
    }
}

fn main() {
    let prots = parse_fasta();
    let densities = prots.iter().map(|prot| {density(prot)}).collect::<Vec<f64>>();
    let mut max_dens: f64 = 0.0;
    let mut idx: usize = 0;
    for (i,dens) in densities.iter().enumerate() {
        if dens > &max_dens {
            max_dens = *dens;
            idx = i;
        }
    }
    write_winner(&prots[idx], densities[idx]);
}
