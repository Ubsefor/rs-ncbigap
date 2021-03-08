use std::env;
use std::fs;
use std::path::Path;
use percentage::Percentage;
use regex::Regex;

fn usage() {
    println!("Usage: rs-ncbigap <genome file path>\nNote, that file must not contain header!");
}

fn version(){
    println!("version: 0.1.0");
}

pub fn count_genes(genome : String, gene : char) -> usize {
    return genome.matches(gene).count();
}

pub fn count_gaps(genome : String) -> usize {
    let re = Regex::new("N{10,}").unwrap();
    return re.find_iter(genome.as_str()).count();
}



fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            usage();
            return;
        },

        2 => {
            if  &args[1] == "-h" {
                usage();
                return;
            }
            if &args[1] == "-v"{
                version();
                return;
            }
            let filepath = &args[1];
            if !(Path::new(filepath).is_file()){
                panic!("Error: couldn't read file at specified filepath!");
            }
            let contents = fs::read_to_string(filepath)
                .expect("Something went wrong reading the file");
            if !(contents.chars().nth(0).unwrap() == 'A' 
                    || contents.chars().nth(0).unwrap() == 'C'
                    || contents.chars().nth(0).unwrap() == 'G' 
                    || contents.chars().nth(0).unwrap() == 'T'
                    || contents.chars().nth(0).unwrap() == 'N') {
                panic!("Error: file seems to contain a header or not a raw genome!")
            }
            println!("Parsing genome in file: {}", filepath);
            let mut size : usize = contents.len();
            if contents.chars().last().unwrap() == '\n' {
                size = contents.len() - 1;
            }
            println!("Total genome size: {}", size);
            let ngenes = count_genes(contents.clone(), 'N');
            println!("Amount of unknown nucleotides, marked by N: {}", ngenes);
            let percent = Percentage::from_decimal(ngenes as f64 /size as f64);
            println!("A percentage of unknown genes in the genome: {}", percent.value());
            println!("Total gaps for N>10: {}", count_gaps(contents));
            return ;
        },
        
        _ => {
            usage();
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts(){
        assert_eq!(count_genes("NNN".to_string(), 'N'), 3);
    }

    #[test]
    fn test_cgaps(){
        assert_eq!(count_gaps("ANNNNNNNNNN".to_string()), 1);
        assert_eq!(count_gaps("AN".to_string()), 0);
        assert_eq!(count_gaps("A".to_string()), 0);
        assert_eq!(count_gaps("ANANA".to_string()), 0);
        assert_eq!(count_gaps("ANNNNNNNNNNANA".to_string()), 1);
        assert_eq!(count_gaps("ANNNNNNNNNNNNNNNNNNNNNNNNNANA".to_string()), 1);
    }
    
}

