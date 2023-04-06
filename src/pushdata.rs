use csv::StringRecord;
use crate::Data;
use mysql::prelude::*;
use mysql::*;
use crate::Reader;
pub mod gettablecol;
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
    //read from csv the column names
   //execute sql statement below
//   let mut querystring:String=String::from("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA='testcsv' AND TABLE_NAME='");
//   querystring.push_str(tablename);
//   querystring.push_str("'");
    //let columnname = conn.query_map("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA='testcsv' AND TABLE_NAME='Data'", |(COLUMN_NAME)| COLUMN_NAME)?;
//    let columnname: Vec<String> = conn.query_map(querystring, |(COLUMN_NAME)| COLUMN_NAME)?;
   //SELECT `COLUMN_NAME`  FROM `INFORMATION_SCHEMA`.`COLUMNS`  WHERE `TABLE_SCHEMA`='testcsv' AND `TABLE_NAME`='Data'; 
 let columname: Vec<String> = gettablecol::get_table_col(&mut conn, &tablename).unwrap();
 println!("{:?}", columname);
 let insertstatement =gettablecol::createinsertstatement(&mut conn, &tablename);
 println!("{}", insertstatement);
//   let mut insertstatement=String::from("INSERT INTO "); 
//   insertstatement.push_str(&tablename);
//   insertstatement.push_str(" (");
//   //Todo combine these two for loops for better efficiency
//    for i in &columnname{
//         insertstatement.push_str(&i);
//         insertstatement.push_str(",");
//    }
//    insertstatement.pop();
//    insertstatement.push_str(") VALUES (");
//    for i in &columnname{
//        insertstatement.push_str(":");
//        insertstatement.push_str(&i);
//        insertstatement.push_str(",");
//    }
//    insertstatement.pop();
//    insertstatement.push_str(")");
//    println!("{:?}", insertstatement);

    //dynamically insert into table tablename based on number of columns in columname variable

    
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
pub fn read_csv(file: &String) -> std::result::Result<(), Box<dyn std::error::Error>> {
    //fn read_csv() ->Vec<Data> {
    let mut rdr = Reader::from_path(file)?;
    //let mut data: Vec<String> = Vec::new();
    let mut data: Vec<Data> = Vec::new();
    //let mut data2 : Vec<InsertData>=Vec::new();
    let mut vecty:Vec<&str>=Vec::new();
    //iterate through every column in csv file
    let mut rdr2=Reader::from_path(file)?;
    
        for result in rdr2.records() {
        let record = result?;

        let columnname = rdr.headers()?;
        
       // let columnames=InsertData{
       //     data:vecty
       // };
        let columncount=columnname.len();
  //      for column in 0..columncount {
        //println!("Column Name: {}", &columnname2[column]);
        //println!("Column Index: {}", column); 
        //let _id = record[column].to_string();

        //data.push(record[column].to_string());
        let id = record[0].parse::<i32>().unwrap();
        let name = record[1].to_string();
        let age = record[2].parse::<i32>().unwrap();
        let address = record[3].to_string();
        let salary = record[4].parse::<i32>().unwrap();
        

       data.push(Data {
            id,
            name,
            age,
            address,
            salary,
        });

    //}

    }
    let mut rdr3=Reader::from_path(file)?;
        let columnname2 = rdr3.headers()?;
        for u in columnname2{

            vecty.push(&u);
        }
    println!("{:?}", data);
    let tablename= std::env::args().nth(2).expect("No Table");
    let connection = crate::database_connection();
    execute_insert(data, tablename,connection, vecty);
    Ok(())
}
