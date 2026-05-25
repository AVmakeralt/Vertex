use crate::backend::compiler::instructions::Instructions;
#[derive(Clone, Debug)]
pub struct ObjFile {
    pub instructions: Vec<Instructions>,
    pub name: String,
    pub imports: Vec<String>,
}
