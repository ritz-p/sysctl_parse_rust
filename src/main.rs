use std::{collections::BTreeMap, env, error::Error, fs::File, io::{BufRead, BufReader}};

use dotenv::dotenv;
fn main() -> Result<(),Box<dyn Error>> {
    dotenv().ok();
    let mut sysctl_map:BTreeMap<usize,(bool,String)> = BTreeMap::new();
    let sysctl_file_path = env::var("SYSCTL_FILE_PATH").expect("SYSCTL_FILE_PATH must be set");
    let sysctl_file_content = read_file(&sysctl_file_path)?;
    for (index,line) in sysctl_file_content.lines().enumerate(){
        let line = line?;
        if line.trim().is_empty(){
            continue;
        }
        let first_char = line.chars().next();
        let is_comment = match first_char{
            Some(';') => true,
            Some('#') => true,
            _ => false,
        };
        sysctl_map.insert(index, (is_comment,line));
    }
    println!("{:?}",sysctl_map);
    Ok(())
}

fn read_file(path: &str) -> Result<BufReader<File>,Box<dyn Error>>{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}