#[allow(dead_code)]
pub struct AAResidue {
    name: String,
    atoms: Vec<u64>,
}

#[allow(dead_code)]
pub struct AAChain {
    id: String,
    segname: String,
    moltype: u32,
    residues: Vec<AAResidue>,
}

#[allow(dead_code)]
pub struct AAMolecule {
    atom_names: Vec<String>,
    atom_coors: Vec<[f64; 2]>,
    residues: Vec<AAResidue>,
    chains: Vec<AAChain>,
}

#[allow(dead_code)]
pub struct AACGResidue {
    res_idx: u32,
    res_name: String,
    atm_name: String,
    atoms: Vec<u64>,
}

#[allow(dead_code)]
pub struct AACGChain {
    first: u32,
    last: u32,
    moltype: u32,
    segname: String,
}
