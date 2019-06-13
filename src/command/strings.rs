use super::Context;
use super::super::storage::object::Object;

use crate::resp;

pub fn get(ctx: &Context) ->resp::Kind {
   // let dict = &ctx.db.dict;
   // let cmd = &ctx.cmd;

   // let key = &cmd.args[0];

   // if let Some(Object::Value(s)) = dict.get(key) {
   //     resp::Kind::BulkString(Some(s.to_vec()))
   // }else {
   //     resp::Kind::BulkString(None)
   // }
   resp::Kind::BulkString(Some("hello".as_bytes().to_vec()))
}

//fn set(ctx: &mut Context) -> resp::Kind {
//    let cmd = &ctx.cmd;
//    let dict = &ctx.db.dict;
//    let (key, val) = (&cmd.args[0], &cmd.args[1]);
//    dict.insert(key.to_vec(), Object::Value(val.to_vec()));
//    resp::Kind::SimpleString("OK".as_bytes().to_vec())
//}