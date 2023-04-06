use csv::StringRecord;
use crate::Data2;
use crate::Data;
use mysql::prelude::*;
use mysql::*;
use crate::Reader;
use crate::Table;
pub mod gettablecol;
pub mod createtablestruct;
#[derive(Debug)] struct InsertData<'a>{
    data: Vec<&'a str>,
}

fn execute_insert(
    data: Vec<Data>,
    //data: &Vec<String>,
    tablename: String,
    mut conn: PooledConn,
    columnames: Vec<&str>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    
    let columname: Vec<String> = gettablecol::get_table_col(&mut conn, &tablename).unwrap();
    println!("{:?}", columname);
    let insertstatement =gettablecol::createinsertstatement(&mut conn, &tablename);
    println!("{}", insertstatement);

    
     conn.exec_batch(
        r"INSERT INTO Data(id, name, age, address, salary)
       VALUES (:id, :name, :age, :address, :salary)",
       data.iter().map(|p| {
            params! {
                "id" => p.id,
                "name" => &p.name,
                "age" => p.age,
                "address" => &p.address,
                "salary" => p.salary,
            }
        }),
    )?;
    //insert into mysql data from data variable into columns in columnname variable

   // conn.exec_batch(
    //   insertstatement, 
        
      //  data.iter().map(|p| {
    //  data.chunks(columnname.len()).map(|p|{
            //let
//                //let mut
     //       params! {
    //for i in columnname.iter(){
//   //             for i in &columnname{
     //           i=>  p.iter().next().unwrap(),
//                }   
//            }
//        }),
//   )?;

    Ok(())
    //todo
}
fn execute_insert2(
    data: Vec<Data2>,
    //data: &Vec<String>,
    tablename: String,
    mut conn: PooledConn,
    columnames: Vec<&str>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    
    let columname: Vec<String> = gettablecol::get_table_col(&mut conn, &tablename).unwrap();
    println!("{:?}", columname);
    let insertstatement =gettablecol::createinsertstatement(&mut conn, &tablename);
    println!("{}", insertstatement);

    
     conn.exec_batch(
        r"INSERT INTO Data(id, name, age, address, salary)
       VALUES (:id, :name, :age, :address, :salary)",
       data.columns.iter().map(|p| {
            params! {
                "id" => p.id,
                "name" => &p.name,
                "age" => p.age,
                "address" => &p.address,
                "salary" => p.salary,
            }
        }),
    )?;
    //insert into mysql data from data variable into columns in columnname variable

   // conn.exec_batch(
    //   insertstatement, 
        
      //  data.iter().map(|p| {
    //  data.chunks(columnname.len()).map(|p|{
            //let
//                //let mut
     //       params! {
    //for i in columnname.iter(){
//   //             for i in &columnname{
     //           i=>  p.iter().next().unwrap(),
//                }   
//            }
//        }),
//   )?;

    Ok(())
    //todo
}
