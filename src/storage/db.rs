use crate::storage::object::Dict;

#[derive(Debug)]
pub struct Db {
    pub dict: Dict,
    pub expires: Dict,
}
impl Db {
    pub fn new() -> Db{
        Db{
            dict: Dict::new(),
            expires: Dict::new()
        }
    }
}
