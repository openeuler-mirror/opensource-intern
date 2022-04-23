use std::process;
//check verison
pub fn check_version(version:&String){
    let bytes = version.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
    let ch:char =item as char;
    if (ch>='0'&&ch<='9')||(ch=='+')||(ch=='-')||(ch=='.')||('a'<=ch&&ch<='z')||('A'<=ch&&ch<='Z')||(ch=='_') {}
    else
    {
        println!("version may be wrong");process::exit(1);
        }
    }
}