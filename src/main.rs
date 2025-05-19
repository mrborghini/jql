use language::import_db;

fn main(){
    println!("Hello, world!");
    let path = "db.jql";
    let data = import_db(path);
    println!("{:?}", data);
}