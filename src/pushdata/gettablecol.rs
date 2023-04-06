use mysql::prelude::*;
use mysql::*;
pub fn get_table_col(conn: &mut PooledConn, table_name: &str)
 -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    //let mut stmt = conn.prepare(sql).unwrap();
 //Result<Vec<String>, mysql::Error> {
    //let mut stmt = conn.prepare(sql).unwrap();
//    let result: Vec<String> = conn.query_map((), |(column_name,)| column_name).unwrap();
    //let columnname: Vec<String> = conn.query_map(querystring, |(COLUMN_NAME)| COLUMN_NAME);
     let mut querystring:String=String::from("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA='testcsv' AND TABLE_NAME='");
   querystring.push_str(table_name.to_string().as_str());
   querystring.push_str("'");
    //let columnname = conn.query_map("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA='testcsv' AND TABLE_NAME='Data'", |(COLUMN_NAME)| COLUMN_NAME)?;
    let columnname = conn.query_map(querystring, |(COLUMN_NAME)| COLUMN_NAME)?;
    //let mut col_vec:Vec<String> = Vec::new();
    //for col in columnname{
    //    col_vec.push(col);
    //}
   Ok((columnname))

}


pub fn createinsertstatement(conn: &mut PooledConn, table_name: &str) -> String
{
    let mut insertstatement = String::from("insert into ");
    insertstatement.push_str(table_name);
    insertstatement.push_str(" (");
    let mut col_vec = get_table_col(conn, table_name).unwrap();
    for col in &col_vec {
        insertstatement.push_str(&col);
        insertstatement.push_str(",");
    }
    insertstatement.pop();
   insertstatement.push_str(") values (");
    for col2 in &col_vec {
        insertstatement.push_str(":");
        insertstatement.push_str(&col2);
        insertstatement.push_str(",");
    }
    insertstatement.pop();
    insertstatement.push_str(")");
    insertstatement


}
