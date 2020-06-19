use std::{fs,io};
use std::path::PathBuf;
use std::ffi::OsString;

fn scan(path:PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut entr = fs::read_dir(&path)?
        .map(|res|res.map(|e|e.path()))
        .collect::<Result<Vec<_>,io::Error>>()?;

    Ok(entr)
}
fn change(a:io::Result<Vec<PathBuf>>) -> Vec<String>{
    let a = a
            .unwrap()
            .into_iter()
            .map(PathBuf::into_os_string)
            .map(OsString::into_string)
            .collect::<Result<_,_>>()
            .unwrap();
a
}
fn do_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool{
    let matching = a
        .iter()
        .zip(b.iter())
        .filter(|&(a,b) |  a==b)
        .count();
    matching == a.len() && matching == b.len()
}
fn check(a: &[String], b: &[String]) -> Vec<String>{
   let mut c:Vec<String> = a
       .iter()
       .chain(b.iter())
       .cloned()
       .collect();
    
    let mut i = 0;
    
    while let Some(s) = c.get(i){
        i += 1;
        
        if c[i..].contains(s){
            let s = s.clone();
            c.retain(|x| x!= &s);
            i -= 1;
        }
    }
c
        }
fn main(){
    let path1 = PathBuf::from(r"/etc/");
    let path2 = PathBuf::from(r"/home/luz");
    let path1 = scan(path1);
    let path1 = change(path1);
    let path2 = scan(path2);
    let path2 = change(path2);
    let sum = do_match(&path1,&path2);
    let mut result = Vec::new();
    match sum {
        true => println!("same"),
        false => result= check(&path1,&path2),
        _=> panic!("wtf"),
    }
    println!("{:?}",result);
}
