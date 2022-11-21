use std::{path::Path, fs::OpenOptions};
use rusqlite::Connection;

use crate::{Rdnf, default::{DEFAULT_DATA_LOCATION, HISTORY_DB_FILE}};

pub struct HistoryCtx{

}
impl HistoryCtx{
    pub fn new(path:&str)->Self{
        let db=Connection::open(path)?;
        // "transactions"

    }
    
}
impl Rdnf{
    pub fn get_history_ctx(&self)->Result<HistoryCtx>{
        let history_db_path=self.rc.cli.installroot.trim_end_matches("/").to_string()
        +DEFAULT_DATA_LOCATION.trim_end_matches("/")+"/"+HISTORY_DB_FILE;
        if !Path::new(history_db_path.as_str()).exists(){
            OpenOptions::new().create(true).open(history_db_path.as_str())?;
        };
    }
    
}
pub fn db_table_exists(db:&Connection){

}