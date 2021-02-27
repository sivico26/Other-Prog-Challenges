pub fn kmer2idx(kmer: &str) -> usize {
    // This index will return an unique index (hash?) given a defined kmer
    // with the following convention: A:0 C:1 G:2 T:3
    // every position on the kmer would e multiplied
    
    let mut idx: usize = 0;
    let max_value = 4usize.pow(kmer.len() as u32);

    for (i, nuc) in kmer.chars().rev().enumerate() {
        let val = match nuc {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            'T' => 3,
            _ => {idx = max_value; break},
        };
        idx += val*(4usize.pow(i as u32));
    }
    
    if idx >= max_value {
        // println!("Invalid kmer. It has not only unambiguous characters");
        return max_value;
    }
    return idx;
}

pub fn idx2kmer(mut idx: usize, k: usize) -> String {
    let mut kmer = String::with_capacity(k);
    let mut v: Vec<usize> = Vec::with_capacity(k);

    while idx > 0 {
        v.push(idx % 4);
        idx /= 4;
    }
    kmer.push_str( &"A".repeat(k - v.len()));

    for val in v.iter().rev() {
        let nuc = match val {
            0 => 'A',
            1 => 'C',
            2 => 'G',
            3 => 'T',
            _ => {println!("Unusual dividend"); '?'},
        };
        
        kmer.push(nuc);
    }

    kmer
}

pub fn freq_kmers(dna: &str, k: usize) -> Vec<u32> {
    // This function goes through a genome an counts kmers of a givven size
    // First we initialize with 0's a sized suitable array + 1 for anomalies
    //let mut freqs: [u32; 4usize.pow(k) + 1] = [0; 4usize.pow(k) + 1];
    // let mut freqs= [0; 4usize.pow(k) + 1];
    let mut freqs = vec![0u32; 4usize.pow(k as u32) + 1]; 

    
    for i in 0..dna.len()-k {
        let idx = kmer2idx(&dna[i..i+k]);
        freqs[idx] += 1;
    }
    return freqs
}