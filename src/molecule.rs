use std::fs;
use std::process;
use std::io::{self, BufRead};

use colored::Colorize;

#[derive(Debug, Default)]
pub struct Structure {
    pub atom_name: Vec<String>,
    pub atom_index: Vec<u32>,
    pub residue_name: Vec<String>,
    pub residue_index: Vec<u32>,
    pub chain_name: Vec<String>,
    pub chain_index: Vec<u32>,
    pub position: Vec<[f32; 3]>,
}

impl Structure {
    pub fn from_pdb_file(pdb: fs::File, verbose: bool) -> Self {
        let mut structure = Structure{..Default::default()};
        let lines = io::BufReader::new(pdb).lines();

        for line in lines {
            if let Ok(line) = line {
                let mut chain_index = 1;
                if line.starts_with("ATOM") {
                    let atom_index = line[6..11].trim().to_string();
                    let atom_index = match atom_index.parse::<u32>() {
                        Ok(index) => index,
                        Err(_) => {
                            if verbose {
                                println!("├─ {} {} {}", "Error:".red().bold(), "Can not parse atom index -> ", atom_index);
                                println!("└─ {} '{}'", "Line:".red().bold(), line);
                            } else {
                                println!("{} {} {}", "Error:".red().bold(), "Can not parse atom index -> ", atom_index);
                                println!("{} '{}'", "Line:".red().bold(), line);
                            }
                            process::exit(1);
                        }
                    };
                    structure.atom_index.push(atom_index);

                    let atom_name = line[12..16].trim().to_string();
                    structure.atom_name.push(atom_name);

                    let residue_name = line[17..20].trim().to_string();
                    structure.residue_name.push(residue_name);

                    let chain_name = line[21..22].trim().to_string();
                    structure.chain_name.push(chain_name);

                    let residue_index = line[22..26].trim().to_string();
                    let residue_index = match residue_index.parse::<u32>() {
                        Ok(index) => index,
                        Err(_) => {
                            if verbose {
                                println!("├─ {} {} {}", "Error:".red().bold(), "Can not parse residue index -> ", residue_index);
                                println!("└─ {} '{}'", "Line:".red().bold(), line);
                            } else {
                                println!("{} {} {}", "Error:".red().bold(), "Can not parse residue index -> ", residue_index);
                                println!("{} '{}'", "Line:".red().bold(), line);
                            }
                            process::exit(1);
                        }
                    };
                    structure.residue_index.push(residue_index);

                    let r_x = line[30..38].trim().to_string();
                    let r_x = match r_x.parse::<f32>() {
                        Ok(x) => x,
                        Err(_) => {
                            if verbose {
                                println!("├─ {} {} {}", "Error:".red().bold(), "Can not parse x position -> ", r_x);
                                println!("└─ {} '{}'", "Line:".red().bold(), line);
                            } else {
                                println!("{} {} {}", "Error:".red().bold(), "Can not parse x position -> ", r_x);
                                println!("{} '{}'", "Line:".red().bold(), line);
                            }
                            process::exit(1);
                        }
                    };

                    let r_y = line[38..46].trim().to_string();
                    let r_y = match r_y.parse::<f32>() {
                        Ok(y) => y,
                        Err(_) => {
                            if verbose {
                                println!("├─ {} {} {}", "Error:".red().bold(), "Can not parse y position -> ", r_y);
                                println!("└─ {} '{}'", "Line:".red().bold(), line);
                            } else {
                                println!("{} {} {}", "Error:".red().bold(), "Can not parse y position -> ", r_y);
                                println!("{} '{}'", "Line:".red().bold(), line);
                            }
                            process::exit(1);
                        }
                    };

                    let r_z = line[46..54].trim().to_string();
                    let r_z = match r_z.parse::<f32>() {
                        Ok(z) => z,
                        Err(_) => {
                            if verbose {
                                println!("├─ {} {} {}", "Error:".red().bold(), "Can not parse z position -> ", r_z);
                                println!("└─ {} '{}'", "Line:".red().bold(), line);
                            } else {
                                println!("{} {} {}", "Error:".red().bold(), "Can not parse z position -> ", r_z);
                                println!("{} '{}'", "Line:".red().bold(), line);
                            }
                            process::exit(1);
                        }
                    };
                    structure.position.push([r_x, r_y, r_z]);

                    structure.chain_index.push(chain_index);
                } else if line.starts_with("TER") {
                    chain_index += 1;
                }
            }
        }

        structure
    }
}
