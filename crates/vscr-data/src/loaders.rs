use std::path::Path;
use serde::Deserialize;

use crate::character_def::CharacterDef;

pub fn load_character_from_file(path: &Path) -> anyhow::Result<CharacterDef> {
    let bytes = std::fs::read(path)?;
    let def: CharacterDef = serde_json::from_slice(&bytes)?;
    Ok(def)
}
