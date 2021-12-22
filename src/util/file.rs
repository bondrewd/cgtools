use std::path::Path;
use std::ffi::OsStr;
use std::fmt;

#[derive(Debug)]
pub enum FileExtension {
    Pdb,
    Cif,
    Gro,
    Unknown,
}

impl fmt::Display for FileExtension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileExtension::Pdb => write!(f, "Protein Data Bank (.pdb)"),
            FileExtension::Cif => write!(f, "Protein Data Bank Extended (.cif)"),
            FileExtension::Gro => write!(f, "Gromacs Coordinate (.gro)"),
            FileExtension::Unknown => write!(f, "Unknown extension")
        }
    }
}

pub fn get_extension_from_filename(filename: &str) -> FileExtension {
    let extension = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str);
    match extension {
        Some("pdb") => FileExtension::Pdb,
        Some("cif") => FileExtension::Cif,
        Some("gro") => FileExtension::Gro,
        _ => FileExtension::Unknown,
    }
}
