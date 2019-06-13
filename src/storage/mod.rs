pub mod db;
pub mod txn;
pub mod object;

use array;
pub struct Storage {
    pub slots: [db::Db; 256]
}

impl Storage{
    pub fn new() ->Storage{
        Storage{
            slots: array::init!([db::Db::new(); 256])
        }
    }
}