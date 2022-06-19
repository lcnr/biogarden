use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;
use allwords::{Alphabet};
use std::error;
use std::cmp;

use super::graph::suffix_tree::SuffixTreeBuilder;
use super::graph::suffix_tree::SuffixTreeEdge;
use super::graph::suffix_tree::SuffixTreeNode;

use crate::ds::sequence::Sequence;
use crate::ds::graph::Graph;
use crate::ds::tile::Tile;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error, BufWriter};
use super::graph::trie::{Trie, TrieNode};

// TODO: Remove later:
use crate::ds::graph::GraphProperties;

// Count number of chars in Sequence sequence
// Return array with numbers representing #occur of given char
// count[0] == count ['A'] and count[23] == count['Z']
pub fn count_nucleotides(seq: &Sequence) -> HashMap<u8, u16> {
    let mut count = HashMap::<u8, u16>::new();
    for c in seq.into_iter() {
        if !count.contains_key(c) {
            count.insert(*c, 1);
        }
        else {
            *count.get_mut(c).unwrap() += 1; 
        }
    }
    count
}

// Transcribe the DNS sequence into RNA
pub fn transcribe_dna(dna: Sequence) -> Sequence {
    let temp = String::from(dna);
    Sequence::from(temp.replace("T", "U"))
}

// Complement the Sequence string by reversing in the first step.
// Swap: 'A' <-> 'T' and 'G' <-> 'C'
pub fn complement_dna(seq: Sequence) -> Sequence {
    let mut t = seq.into_iter().rev().map(|c| c as char).collect::<String>();
    // A <-> T
    t = t.replace("A", "X");
    t = t.replace("T", "A");
    t = t.replace("X", "T");
    // G <-> C
    t = t.replace("G", "X");
    t = t.replace("C", "G");
    t = t.replace("X", "C");
    Sequence::from(t)
}

// Percentage of G/C nucleotides in sequence
// Return percentage value
pub fn gc_content(seq: &Sequence) -> f64 {
    let mut gc_count : u32 = 0;
    for c in seq {
        if *c == b'G' ||  *c == b'C' {
            gc_count += 1;
        }
    }
    gc_count as f64 / seq.len() as f64
}

// Make parametrizable number of sequences to find
pub fn translate_rna(rna: Sequence) -> Vec<Sequence> {

    let mut proteins : Vec<Sequence> = vec![];

    // mRNA <-> amino-acid translation table (codon table)
    let codon_table = HashMap::from([   
        ("UUU", "F"),    ("CUU", "L"),   ("AUU", "I"),   ("GUU", "V"),
        ("UUC", "F"),    ("CUC", "L"),   ("AUC", "I"),   ("GUC", "V"),
        ("UUA", "L"),    ("CUA", "L"),   ("AUA", "I"),   ("GUA", "V"),
        ("UUG", "L"),    ("CUG", "L"),   ("AUG", "M"),   ("GUG", "V"),
        ("UCU", "S"),    ("CCU", "P"),   ("ACU", "T"),   ("GCU", "A"),
        ("UCC", "S"),    ("CCC", "P"),   ("ACC", "T"),   ("GCC", "A"),
        ("UCA", "S"),    ("CCA", "P"),   ("ACA", "T"),   ("GCA", "A"),
        ("UCG", "S"),    ("CCG", "P"),   ("ACG", "T"),   ("GCG", "A"),
        ("UAU", "Y"),    ("CAU", "H"),   ("AAU", "N"),   ("GAU", "D"),
        ("UAC", "Y"),    ("CAC", "H"),   ("AAC", "N"),   ("GAC", "D"),
        ("UAA", "Stop"), ("CAA", "Q"),   ("AAA", "K"),   ("GAA", "E"),
        ("UAG", "Stop"), ("CAG", "Q"),   ("AAG", "K"),   ("GAG", "E"),
        ("UGU", "C"),    ("CGU", "R"),   ("AGU", "S"),   ("GGU", "G"),
        ("UGC", "C"),    ("CGC", "R"),   ("AGC", "S"),   ("GGC", "G"),
        ("UGA", "Stop"), ("CGA", "R"),   ("AGA", "R"),   ("GGA", "G"),
        ("UGG", "W"),    ("CGG", "R"),   ("AGG", "R"),   ("GGG", "G") 
    ]);
    // Container for final result of transcription
    let mut amino_acid = String::new();
    // Run the translation 
    let s = String::from(rna);
    let mut z = s.chars().peekable();
    // Iterate until end of strand is reached
    while z.peek().is_some() {
        amino_acid.clear();
        // Iterate over strand until start codon found
        while z.peek().is_some() {
            // Take 3 characters from strand, that denote codon
            let chunk: String = z.by_ref().take(3).collect();
            // Check for start codon
            if chunk == "AUG"{
                amino_acid.push_str(codon_table.get(&chunk as &str).unwrap());
                break;
            }
        }
        // Copy current iterator to resume search for start codon at that position
        let mut zi = z.clone(); 
        // Decode until stop codon reached
        while zi.peek().is_some() {
            // Take 3 characters from strand, that denote codon
            let chunk: String = zi.by_ref().take(3).collect();
            match codon_table.get(&chunk as &str) {
                Some(value) => {
                    // If stop codon reached, store current protein strand and proceed 
                    if value == &"Stop"{
                        proteins.push(Sequence::from(amino_acid.clone()));
                        break;
                    }
                    else {
                        amino_acid.push_str(value);
                    }
                },
                None => {
                    print!("value: {}\n", &chunk);
                    println!("Codon not found in codon table.");
                    break;
                }
                
            }
        }
    }
    proteins
}

pub fn hamming_distance(s1: &Sequence, s2: &Sequence) -> usize {
    s1.into_iter().zip(s2.into_iter()).filter(|(a, b)| a != b).count()
}

/// Obtain tuple containing edit-distance and edit-alignment of two genetic sequences.
///   
/// The Hamming distance provides a way to model point mutations transforming one genetic string into another. 
/// In practice, point mutations include insertions and deletions in addition to replacements only.
/// This can produce genetic sequence that vary in length, and cannot be compared using hamming distance.
/// In such scenarios, a measure of the minimum number of replacements / insertions / deletions between two sequences, is provided by edit distance.
/// The edit distance provides additional information about the type and location where mutations have occurred.
/// 
/// # Arguments
///
/// * `seq1` - first sequence to calculate edit alignment
/// * `seq2` - second sequence to calculate edit alignment
/// 
pub fn edit_distance(seq1: &Sequence, seq2: &Sequence) -> usize {

    // Data containers 
    let mut edit_distance = 0_u128;
    let mut memo = vec![vec![0_u128; seq2.len() + 1 ]; seq1.len() + 1];
    let mut action_matrix = vec![vec![0_u8; seq2.len() + 1 ]; seq1.len() + 1];
    // let mut count = vec![vec![0_u128; seq2.len() + 1 ]; seq1.len() + 1];

    // initialize table
    for i in 0..(seq1.len() + 1) {
        memo[i][0] = i as u128;
        // count[i][0] = 1;
    }
    for j in 0..(seq2.len() + 1) {
        memo[0][j] = j as u128;
        // count[0][j] = 1;
    }

    // Calculate edit-distance dp table
    for i in 1..seq1.len()+1 {
        for j in 1..seq2.len()+1 {
            let minimum = cmp::min(memo[i-1][j-1] + ((seq1[i-1] != seq2[j-1]) as u128), cmp::min(memo[i][j-1] + 1, memo[i-1][j] + 1));
            // Evaluate whether edit, insert, replace
            if minimum == memo[i-1][j-1] + ((seq1[i-1] != seq2[j-1]) as u128) {
                action_matrix[i][j] = b'R';
                // count[i][j] += count[i-1][j-1] % 134_217_727;
            }
            if minimum == memo[i-1][j] + 1 {
                action_matrix[i][j] = b'D';
                // count[i][j] += count[i-1][j] % 134_217_727;
            } 
            if  minimum == memo[i][j-1] + 1 {
                action_matrix[i][j] = b'I';
                // count[i][j] += count[i][j-1] % 134_217_727 ;
            } 
            
            // Update edit distance in memoization table
            memo[i][j] = minimum % 134_217_727;
        }
    }

    memo[seq1.len()][seq2.len()] as usize
}

pub fn open_reading_frames(dna: &Sequence) -> Vec<Sequence> {

    let mut reading_frames : Vec<Sequence> = vec![];

    let mut strands : Vec<Sequence> = vec![];
    strands.push(dna.clone());
    strands.push(complement_dna(dna.clone()));

    for strand in &strands {
        for i in 0..3 {
            let mut temp: Sequence = strand.clone();
            temp.chain.drain(0..i);
            temp = transcribe_dna(temp);
            reading_frames.extend(translate_rna(temp));
        }    
    }
    reading_frames
}

pub fn infer_number_rna(protein: &Sequence) -> u128 {

    let codon_combs: HashMap<u8, u128> = HashMap::from([   
        (b'F', 2),   (b'I', 3),   (b'V', 4),   (b'L', 6),   
        (b'S', 6),   (b'P', 4),   (b'M', 1),   (b'T', 4),   
        (b'A', 4),   (b'Y', 2),   (b'-', 3),   (b'H', 2),   
        (b'N', 2),   (b'D', 2),   (b'Q', 2),   (b'K', 2),   
        (b'E', 2),   (b'C', 2),   (b'G', 4),   (b'R', 6),      
        (b'W', 1)
    ]);

    // Initialize with 3 as for number of STOP codons
    let mut rna_combinations : u128 = 3;
    // Compute number of combinations
    for amino in protein {
        rna_combinations = (rna_combinations * codon_combs.get(amino).unwrap()) % 1000000;
    }   
    rna_combinations
}

pub fn weighted_mass(protein: &Sequence) -> f64 {

    let monoisotopic_mass_table : HashMap<u8, f64> = HashMap::from([   
        (b'F', 147.06841),   (b'I', 113.08406),   (b'V', 99.06841),   (b'L', 113.08406),   
        (b'S', 87.03203),    (b'P', 97.05276),    (b'M', 131.04049),  (b'T', 101.04768),   
        (b'A', 71.03711),    (b'Y', 163.06333 ),  (b'-', 0.0),        (b'H', 137.05891),   
        (b'N', 114.04293),   (b'D', 115.02694),   (b'Q', 128.05858),  (b'K', 128.09496),   
        (b'E', 129.04259),   (b'C', 103.00919),   (b'G', 57.02146),   (b'R', 156.10111),      
        (b'W', 186.07931)
    ]);

    let mut mass : f64 = 0.0;
    for amino in protein.into_iter() {
        mass += monoisotopic_mass_table.get(amino).unwrap(); 
    }
    mass
}

pub fn knuth_morris_pratt(seq: &Sequence, pat: &Sequence) -> Vec<usize> {
   
    let seq = seq.to_string().into_bytes();
    let pat = pat.to_string().into_bytes();

    // Build the partial match table
    let mut partial = vec![0];
    for i in 1..pat.len() {
        let mut j = partial[i - 1];
        while j > 0 && pat[j] != pat[i] {
            j = partial[j - 1];
        }
        partial.push(if pat[j] == pat[i] { j + 1 } else { j });
    }

    // Read 'string' to find 'pattern'
    let mut ret = vec![];
    let mut j = 0;

    for (i, &c) in seq.iter().enumerate() {
        while j > 0 && c != pat[j] {
            j = partial[j - 1];
        }
        if c == pat[j] {
            j += 1;
        }
        if j == pat.len() {
            ret.push(i + 1 - j);
            j = partial[j - 1];
        }
    }
    ret
}

// Find all reverse-palindromes within seq of n <= length <= m
// Return tuples containing position and length of each palindrome O(n^3)?
pub fn reverse_palindromes(seq: &Sequence, n: usize, m: usize) -> Vec<(usize, usize)>{

    let mut palindromes : Vec<(usize, usize)> = vec![];
    let complements = HashMap::from([(b'A', b'T'), (b'T', b'A'),
                                     (b'G', b'C'), (b'C', b'G')]);

    // iterate over every offset within the initial string
    for i in 0..seq.len() {
        // iterate over possible lengths of palindromic substrings
        for j in n..(m+1) {
            // break if potential substring cannot fit 
            if i + j > seq.len() {
                break;
            } 
            // check if substring with length `j` at offset `i` 
            // is a reverse palindrome
            let mut is_palindrome = true;
            for k in 0..j {
                if seq.chain[i+k] != complements[&seq.chain[i+j-1-k]] {
                    is_palindrome = false;
                    break;
                }
            }
            // append (offset, length) into result set
            if is_palindrome {
                palindromes.push((i+1,j));
            }
        }    
    }
    palindromes
}

pub fn transition_transversion_ratio(s1: &Sequence, s2: &Sequence) -> f32 {
    let s1 = s1.to_string();
    let s2 = s2.to_string();
    let missmatch_iter = s1.chars().zip(s2.chars()).filter(|&(a, b)| a != b);
    let hamming = missmatch_iter.clone().count() as f32;
    let mut transitions : f32 = 0.0;
    for (a, b) in missmatch_iter {
        // C <-> T 
        if (a == 'C' && b == 'T') || (a == 'T' && b == 'C'){
            transitions += 1.0;
        }
        // A <-> G
        if (a == 'A' && b == 'G') || (a == 'G' && b == 'A'){
            transitions += 1.0;
        }
    }
    let transversions = hamming  - transitions;
    transitions / transversions
}

pub fn rna_splice(mut pre_rna: Sequence, introns: &Tile) -> Sequence {

    for intr in introns {
        let res = knuth_morris_pratt(&pre_rna, intr);
        for index in res {
            pre_rna.chain.drain(index..(index + intr.len()));
        }
    }
    pre_rna
}

pub fn subsequences(a: &Sequence, b: &Sequence, limit: Option<usize>) -> Vec<Vec<usize>> {

    let mut result = vec![];
    let mut temp = Vec::<usize>::new();
    let a_idx: usize = 0;
    let b_idx: usize = 0; 

    pub fn subsequences_recursive( a: &Sequence, a_idx: usize, b: &Sequence, b_idx: usize, 
                                     temp: &mut Vec<usize>, result: &mut Vec<Vec<usize>>,
                                     limit: Option<usize>) 
    {
        if b_idx == b.len() {
            result.push(temp.clone());
            return;
        }
        for i in a_idx..a.len() {
            if limit.is_some() && result.len() == limit.unwrap() {
                return;
            }
            if b[b_idx] == a[i] {
                temp.push(i);
                subsequences_recursive(a, i+1, b, b_idx+1, temp, result, limit);
                temp.pop();
            }
        }   
    }

    subsequences_recursive(a, a_idx, b, b_idx, &mut temp, &mut result, limit);
    return result;
}

pub fn longest_increasing_subsequence(seq: &[u64]) -> Vec<u64> {

    let mut lis = vec![0; seq.len()];
    let mut pointers = vec![0; seq.len()];

    let mut max_idx : usize = 0;
    let mut max_len : usize = 0;

    lis[0] = 1;
    max_idx = 0;

    for i in 1..lis.len() {

        lis[i] = 1;
        pointers[i] = i;

        for j in 0..i {

            // TODO: Make more generic
            // Pass comparator as parameter
            if seq[i] > seq[j] && lis[i] < lis[j] + 1 {
            
                lis[i] = lis[j] + 1;
                pointers[i] = j;
            
                if lis[i] > max_len {
                    max_idx = i;
                    max_len = lis[i];
                }
            }
        }
    }

    let mut result : Vec<u64> = vec![];

    while max_len > 0 {
        result.push(seq[max_idx]);
        max_idx = pointers[max_idx];
        max_len -= 1;
    }

    result.reverse();
    result

}

pub fn sort_lexicographically(sequences: &Tile, alphabet: &[u8]) -> Tile {

    let mut trie_builder = Trie::new(alphabet);
    let trie = trie_builder.build(sequences).unwrap();

    fn walk_trie_rec(trie: &Graph<TrieNode, u8>, node_id: u64, sorted: &mut Tile) {
        
        if trie.out_neighbors(node_id).count() == 0 {
            return;
        }

        let node = trie.get_node(&node_id);
 
        for c in node.data.children.iter() {
            if *c != -1 {
                if trie.get_node(&(*c as u64)).data.ending == true {
                    let substrings = &trie.get_node(&(*c as u64)).data.substring;
                    substrings.iter().for_each(|s| sorted.push(Sequence::from(s)));
                }
    
                walk_trie_rec(trie, *c as u64, sorted);
            }
        }
    }

    let mut sorted = Tile::new();
    let root = trie.get_root().unwrap();
    walk_trie_rec(trie, root, &mut sorted);

    sorted
}

pub fn longest_common_subsequence(seq1: &Sequence, seq2: &Sequence) -> Sequence {

    let mut match_table = vec![vec![0_usize; seq2.len() + 1 ]; seq1.len() + 1];
    let mut prev_table = vec![vec![(0_usize, 0_usize); seq2.len() + 1 ]; seq1.len() + 1];

    for i in 1..(seq1.len()+1) {
        for j in 1..(seq2.len()+1) {
            if seq1[i-1] == seq2[j-1] {
                match_table[i][j] = match_table[i-1][j-1] + 1;
                prev_table[i][j] = (i-1, j-1);
            }
            else {
                if match_table[i-1][j] > match_table[i][j-1] {
                    match_table[i][j] = match_table[i-1][j];
                    prev_table[i][j] = (i-1, j);
                }
                else {
                    match_table[i][j] = match_table[i][j-1];
                    prev_table[i][j] = (i, j-1);  
                }
            }
        } 
    }

    let mut lcs = Sequence::new();

    let mut i = seq1.len();
    let mut j = seq2.len();

    while match_table[i][j] != 0 {
        let i_next = prev_table[i][j].0;
        let j_next = prev_table[i][j].1;
        if i_next==i-1 && j_next==j-1 {
            lcs.push(seq1[i_next]);
        }
        i = i_next;
        j = j_next
    }

    lcs.reverse();
    lcs
}

pub fn k_mer_composition(seq: &Sequence, k: usize, alphabet: &[u8]) -> Vec<usize> {

    // Generate all possible k-mers from alphabet
    let mut kmers = Tile::new();
    let a = Alphabet::from_chars_in_str(std::str::from_utf8(alphabet).unwrap()).unwrap();
    let words = a.all_words(Some(k)).filter(|x| x.len() == k);
    for a in words {
        kmers.push(Sequence::from(a));
    }

    // Sort k-mers according to ordering from alphabet
    kmers = sort_lexicographically(&kmers, alphabet);

    // Calculate k-mer composition
    let mut kmer_composition = vec![];
    for kmer in &kmers {
        let pos = knuth_morris_pratt(seq, kmer);
        kmer_composition.push(pos.iter().count());
    }

    kmer_composition
}

pub fn longest_common_supersequence(seq1: &Sequence, seq2: &Sequence) -> Sequence {

    let lcs = longest_common_subsequence(seq1, seq2);
    
    let mut superseq = Vec::<u8>::new();

    let mut s1 = seq1.into_iter();
    let mut s2 = seq2.into_iter();

    for c in lcs {

        loop {
            match s1.next() {
                Some(x) if *x != c => {
                    superseq.push(*x);
                }
                _ => {
                    break;
                }
            }
        }

        loop {
            match s2.next() {
                Some(x) if *x != c => {
                    superseq.push(*x);
                }
                _ => {
                    break;
                }
            }
        }

        superseq.push(c);
    }

    superseq.extend(s1);
    superseq.extend(s2);

    Sequence::from(superseq.as_slice())
}


pub fn protein_from_prefix_spectrum(spec: Vec<f32>) -> Sequence {

    let monoisotopic_mass_table : HashMap<u8, f32> = HashMap::from([   
        (b'F', 147.06841),   (b'I', 113.08406),   (b'V', 99.06841),   (b'L', 113.08406),   
        (b'S', 87.03203),    (b'P', 97.05276),    (b'M', 131.04049),  (b'T', 101.04768),   
        (b'A', 71.03711),    (b'Y', 163.06333 ),  (b'-', 0.0),        (b'H', 137.05891),   
        (b'N', 114.04293),   (b'D', 115.02694),   (b'Q', 128.05858),  (b'K', 128.09496),   
        (b'E', 129.04259),   (b'C', 103.00919),   (b'G', 57.02146),   (b'R', 156.10111),      
        (b'W', 186.07931)
    ]);

    let mut result = Sequence::new();

    for i in 1..spec.len() {
        let mut diff = spec[i] - spec[i-1];
        let x = monoisotopic_mass_table.iter()
                        .find(|(key, value)| (*value - diff).abs() < 0.01).unwrap();
        result.push(*x.0);
    }

    result
}


/// Returns tuples denoting error corrections that can be applied to a number of input reads.
///   
/// Genome sequencers use a chemical procedure to obtain reads from provided biomaterial.
/// Due to the error-prone character of this approach, multiple reads of the same region are usually taken.
/// Errors can be corrected by splitting the obtained set into correct and faulty reads first.
/// If a read or its complement appears in the set at least `split_margin` times, it is regarded as correct.
/// Faulty reads are corrected based on their hamming distance to one of the correct reads.
/// 
/// # Arguments
///
/// * `reads` - tile containing the analyzed reads
/// * `split_margin` - number of reads/complements required to treat a read as correct
/// * `hd_margin` - hamming distance used for matching faulty reads to correct ones
/// 
pub fn correct_read_errors(reads: &Tile, split_margin: usize, hd_margin: usize) -> Vec<(Sequence, Sequence)> {
    
    // Count number of repeated reads and complements 
    let mut read_counter  = HashMap::<Sequence, usize>::new();
    for read in reads {
        // Insert read if not present already and increment count
        *read_counter.entry(read.clone()).or_insert(0) += 1;
        // Handle complement case
        let complement = complement_dna(read.clone());
        if read_counter.contains_key(&complement) {
            *read_counter.get_mut(&complement).unwrap() += 1;
            *read_counter.get_mut(read).unwrap() += 1;
        }
    }

    // Split according to split margin
    let mut correct_reads = HashSet::<Sequence>::new();
    let mut faulty_reads = HashSet::<Sequence>::new();
    read_counter.iter().for_each(|(fr, cnt)| {
        if *cnt >= split_margin {  
            correct_reads.insert(fr.clone()) 
        } else { 
            faulty_reads.insert(fr.clone()) 
        };
    });
    
    // Compute corrections satisfying hamming margin, applicable to faulty reads 
    let mut corrections = Vec::<(Sequence, Sequence)>::new();
    for fr in faulty_reads.iter() {
        // Find correct reads/complements satisfying `H(x) <= hamming_distance_margin`
        for cr in correct_reads.iter() {
            // H(x) <= hamming_distance_margin
            if hamming_distance(fr, cr) <= hd_margin {
                corrections.push((fr.clone(), cr.clone()));
                break;
            }
            // H(complement(x)) <= hamming_distance_margin
            let complement = complement_dna(cr.clone());
            if hamming_distance(fr, &complement) == hd_margin {
                corrections.push((fr.clone(), complement));
                break;
            }
        }
    }

    corrections
}

pub fn longest_common_substring(matrix: &Tile, bound: usize) -> Sequence {

    let alphabet = HashSet::<u8>::from([b'A', b'C', b'T', b'G']);
    let mut suffix_sequence = Sequence::new();

    // Transform set of sequences into one global search string
    // Separate words using any characters that are not members of the alphabet
    let mut separator = 0; 
    for a in matrix {
        suffix_sequence.extend(a.clone());
        suffix_sequence.push(separator);
        separator += 1;
        while alphabet.contains(&separator) { separator += 1; }
    }

    // Build suffix tre using ukonnen's algorithm in O(n)
    let mut ukonnen_builder = SuffixTreeBuilder::new(alphabet);
    let graph = ukonnen_builder.build(&suffix_sequence);

    // DFS
    // Finds the longest common substring
    let mut lcs = Sequence::new();
    let mut max_len = 0;
    let mut stack = Vec::<(u64, Sequence)>::new();
    stack.push((graph.get_root().unwrap(), Sequence::new()));

    while !stack.is_empty() {
        
        let (cur_node_id, cur_sequence)  = stack.pop().unwrap();

        if cur_sequence.len() > max_len {
            max_len = cur_sequence.len();
            lcs = cur_sequence.clone();
        }

        for eid in graph.out_edges(cur_node_id) {
        
            let rs_i = graph.get_node(&graph.get_edge(eid).end).data.reachable_suffixes.iter();

            if rs_i.clone().filter(|x| {**x == 0}).count() == 0 && rs_i.sum::<u64>() >= bound as u64 {
        
                let start = graph.get_edge(eid).data.as_ref().unwrap().suffix_start;
                let stop = graph.get_edge(eid).data.as_ref().unwrap().suffix_stop;

                let mut t = cur_sequence.clone();
                for i in start..stop+1 { t.push(suffix_sequence[i as usize]); }
                stack.push((graph.get_edge(eid).end, t));
            }
        }
    }

    lcs
}

pub fn linguistic_complexity(seq: Sequence) -> f32 {

    // Define alphabet
    let alphabet = HashSet::<u8>::from([b'A', b'C', b'T', b'G']);
    let alphabet_len = alphabet.len();

    // Add the strings to tree and traverse from root to node. 
    // Each root to node path will denote suffixes of a string. 
    let mut ukonnen_builder = SuffixTreeBuilder::new(alphabet);
    let graph = ukonnen_builder.build(&seq);

    // Count unique substrings
    // All the prefixes of these suffixes are unique substrings.
    // Their number can be obtained by summing-up the length of all edges
    let mut num_substrings = 0;
    for eid in graph.edges() {
        let start = graph.get_edge(eid).data.as_ref().unwrap().suffix_start;
        let stop = graph.get_edge(eid).data.as_ref().unwrap().suffix_stop;
        num_substrings += stop - start + 1;
    }

    // The maximum number of k-length substrings for n-letter string is either:
    // limited by the number of substrings that can be formed from a given alphabet (4^k)
    // or by the number of k-windows that can be shifted within n-length string
    let mut max_complexity = 0;
    for k in 1..seq.len()+1 {
        // Perform 4^k only when 4^k < seq.len(), however 4^k test can result in overflow
        // Use k < log4(seq.len()) instead. Convert log4(seq.len()) => log10(seq.len())/log10(4) 
        // This will increase accuracy as log10 is better: 
        // Check -> https://doc.rust-lang.org/std/primitive.f64.html#method.log
        if (k as f32) < ((seq.len() as f32).log10() / (4.0_f32).log10()) {
            max_complexity += u128::pow(alphabet_len as u128, k as u32);
        }
        else {
            max_complexity += (seq.len() - k + 1) as u128;
        }

    }
    
    num_substrings as f32 / max_complexity as f32
}

pub fn generate_k_mers(seq: &Sequence, k: usize) -> Vec<Sequence> {

    // TODO: return proper error
    if k > seq.len() {
        panic!("LEN(SEQ1) !< LEN(SEQ2)");
    }

    (0..seq.len()-k+1).into_iter()
                    .map(|i| Sequence::from(&seq[i..k+i]) )
                    .collect()
}




#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_count_nucleotides() {
    //     let input = Sequence::from("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCT\
    //                               CTGTGTGGAATTAAAAAAAGAGTGTCTGATGCAGC");
    //     assert_eq!([20, 12, 17, 21], count_nucleotides(&input));
    // }

    #[test]
    fn test_transcribe_dna() {
        let input = Sequence::from("GATGGAACTTGACTACGTAAATT");
        let result = Sequence::from("GAUGGAACUUGACUACGUAAAUU");
        assert_eq!(result, transcribe_dna(input))
    }

    #[test]
    fn test_complement_dna() {
        let input = Sequence::from("AAAACCCGGT");
        let result = Sequence::from("ACCGGGTTTT");
        assert_eq!(result, complement_dna(input));
    }

    #[test]
    fn test_gc_content() {
        let input = Sequence::from("CCTGCGGAAGATCGGCACTAGAATAGCCAG\
                                    AACCGTTTCTCTGAGGCTTCCGGCCTTCCC");
        let result : f64 = 0.5833333333333334;
        assert_eq!(result, gc_content(&input));
    }

    #[test]
    fn test_translate_rna() {
        let input = Sequence::from("AUGGCCAUGGCGCCCAGAACUGAGA\
                                    UCAAUAGUACCCGUAUUAACGGGUGA");
        let result = Sequence::from("MAMAPRTEINSTRING");
        assert_eq!(result, *translate_rna(input).first().unwrap());
    }

    #[test]
    fn test_hamming_distance() {
        let input1 = Sequence::from("GAGCCTACTAACGGGAT");
        let input2 = Sequence::from("CATCGTAATGACGGCCT");
        let result : u32 = 7;
        assert_eq!(result, hamming_distance(&input1, &input2));

    }

    #[test]
    fn test_substring_positions() {
        let seq = Sequence::from("GATATATGCATATACTT");
        let pat = Sequence::from("ATAT");
        assert_eq!(vec![1, 3, 9], knuth_morris_pratt(&seq, &pat));
    }

    #[test]
    fn test_transition_transversion_ratio() {
        let a = Sequence::from("GCAACGCACAACGAAAACCCTTAGGGACTGGATTATTTCGT\
                                GATCGTTGTAGTTATTGGAAGTACGGGCATCAACCCAGTT");
        let b = Sequence::from("TTATCTGACAAAGAAAGCCGTCAACGGCTGGATAATTTCGC\
                                GATCGTGCTGGTTACTGGCGGTACGAGTGTTCCTTTGGGT");
        let ratio = transition_transversion_ratio(&a, &b);
        assert_eq!(1.21428571429, ratio);
    }

    #[test]
    fn test_shortest_supersequence() {
        let s2 = Sequence::from("TTATGTGATATCCCCGCTTCTCACAATGCTCTTAGTTTACCTC\
                                 GAACTAAGTCTGATCGCAGCGGCCGGTATTCCTTTCTACGCG");
        let s1 = Sequence::from("GCTCACGGATTCGAAAGTCGAGTGTCCCCCAGCTGGATGCATTCTT\
                                 TGGGAGTGGCCAAGGAGGGTTATCAGAAGAACAGATTAATTTG");
        let res = Sequence::from("GCTCTACTGTGATATCCCCGCTTCTCACAATGCTCGTTAGTGT\
                                  TACCTCCCAGAACTGGATGCAGTTCTTTGGGAGTGGCGCAAGC\
                                  GAGCCGGTTATTCAGAAGAACAGATTAATCTTACGCG");
        assert_eq!(res, longest_common_supersequence(&s1, &s2));
    }
}