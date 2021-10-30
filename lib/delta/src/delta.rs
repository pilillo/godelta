extern crate deltalake;

use std::ffi::CStr;
use deltalake

// https://docs.rs/deltalake/0.4.0/deltalake/fn.open_table.html
#[no_mangle]
pub extern "C" fn open_table(table_uri: *const libc::c_char) {
    let table = deltalake::open_table("./tests/data/simple_table").await.unwrap();
    //println!("{}", table.get_files());
    return table.get_files();
}
