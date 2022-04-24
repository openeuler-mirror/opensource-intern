use std::process;
use glob::glob;
//return information after idex 
pub fn find_information (s:&String,index:usize) ->String {
    let mut  st:usize  =0;
    let mut  fin:usize  =0;
    let ch:&str=&s[index..];
    let Str:String =ch.to_string();
    let bytes = Str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
            if fin!=0&&st!=0 {break;}
            if item as char ==' '
            { continue;}
            else if (item as char=='\n'||item as char==' '||item as char=='\t')&&st!=0 {
            fin=i;
            }
            else if st==0&&item as char !=' '&&item as char !='\t' {
            st=i;
            }
    }
    let fnstr:&str=&Str[st..fin];
    let res:String= fnstr.to_string();
    res

}
//
pub fn find_exist_once(Pname:&String,Version:&String)->(bool,bool,String){
    let pname:&str=&Pname[..];
    let version:&str=&Version[..];
    let plen:usize =pname.to_string().chars().count();
    let mut tag:bool = false;

    //Determine if a suspicious string is found
    let mut tag2:bool = true;
    let mut ne:String=Default::default();
    match version.find(&pname) {
        Some(index) => {
        // To determine whether the name  followed by a number
        let jud:&str=&version.to_string()[index+plen+1..];
        let Jud:String=jud.to_string();
        ne=Jud.to_string();
        let bytes = Jud.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item as char>='0'&&item as char<='9'
            {tag=true;}
            break;//loop is done once just only
        }
         //Determine if it is preceded by a carriage return

          if index!=0 {
           let jud2:&str=&version.to_string()[index-1..];
           let Jud2:String=jud2.to_string();
           let bytes2 = Jud2.as_bytes();
           for (i, &item) in bytes2.iter().enumerate() {
              if item as char !='\n'
              {tag=false;}
             break;//loop is done once just only
              }
          }
        }
        None => {
            println!("not match package name",);
            tag2=false;
        }
    }
    (tag,tag2,ne)
}


//Ask if this package is available in the version image
pub fn find_exist(Pname:&String,Version:&String)->bool{
    let mut tag:(bool,bool,String)=(false,true,Version.to_string());


    //Judgement if there are still suspicious items
    while(tag.1==true){
        tag=find_exist_once(&Pname,&tag.2);
        if tag.0 == true{return true;}
    }
    tag.0
}

//Match the version in the spec
pub fn match_version(Spec:&String,version1:&String)->String{//
   
    let spec:&str=&Spec[..];
    let version:&str=&version1[..];
    let mut mas :String=Default::default();
    let mut Version :String=Default::default();
    match spec.find(&version) {
    Some(index) => {
    
    //mas is the information returned, first determining if it is a global variable like%{}
    mas=find_information(&spec.to_string(),index+8);
    let m_len=&mas.len();

    //Used to determine if a string contains a global variable 

    let mut tag1:bool=false;
    let mut tag2:bool=false;
    let mut tag3:bool=false;

    //mark global variable 
    let mut tag4:bool=false;

    //mark can find global variable 
    let mut tag:(bool,bool)=(true,true);
    let mut p1:usize=0;
    let mut p2:usize=0;
    for (i, &item) in mas.as_bytes().iter().enumerate() {
         if item as char=='%' {tag1=true;}
         else if item as char=='{'{tag2=true;p1=i}
         else if item as char=='}'{tag3=true;p2=i}
         if tag1==true&&tag2==true&&tag3==true {tag4=true;}
    }
    if tag4==true{//turn to be global variable
        let mas2:&str=&mas[p1+1..p2];

        let mut de_name:String="".to_string();
        de_name.push_str(mas2);
        let de_len=&de_name.to_string().len();

        match spec.find(&de_name[..]){
            Some(index)=>
            {
                if p1>0 {
                    Version=mas[..p1-1].to_string();
                }
                Version.push_str(&find_information(&spec.to_string(),index+de_len)[..]);
               
                if p2<m_len-1 {
                    Version.push_str(&mas[p2+1..]);

                }

            }
            None=>{println!("not match de_name");tag.0=false;}
        }
        if tag.0==false {println!("can't find global variable");process::exit(1);}

    }
    else {
    Version=mas;
    }
    }
    None => {
            println!("not match version")
            }
    }
       
    Version
}

//returns the file name of the judgment according to list.txt , cnt
pub fn find_cname(list:&String,cnt:usize)->String{
    let mut cnt_str:String=cnt.to_string();
    cnt_str.push_str(" ");
    let num_str:&str=&cnt_str[..];
    let List:&str =&list[..];
    let mut cname:String=Default::default();
    match List.find(&num_str) {
    Some(index) => {
      cname=find_information(&list,index+cnt_str.len()-1);
     }
    None => {
            println!("not match cname")
            }
    }
    cname
}

////returns the path to the determined spec file
pub fn find_path(path:&String)->String{
    let mut Path:String=Default::default();

    for entry in glob(&path).expect("Failed to read glob pattern") {
    match entry {
        Ok(path) =>{ Path=path.display().to_string();},
        Err(e) => println!("{:?}", e),
    }
    }
    Path
}
