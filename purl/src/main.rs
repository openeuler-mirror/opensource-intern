extern crate packageurl;
use std::str::FromStr;
use packageurl::PackageUrl;
use std::string::ToString;
use std::fs;
use std::io::Write;
use std::fs::OpenOptions;
use glob::glob;
use git2::Repository;
use std::process;

fn Find_massage (s:&String,index:usize) ->String {//返回切片后一个空格一个回车的信息 //
    let mut  st:usize  =0;
    let mut  fin:usize  =0;
    let ch:&str=&s[index..];
    let Str:String =ch.to_string();
    let bytes = Str.as_bytes();
   // for (i, &item) in bytes.iter().enumerate() {
   //     println!("{} {}", i, item as char);
   // }
    for (i, &item) in bytes.iter().enumerate() {
            if fin!=0&&st!=0 {break;}
            if item as char ==' '
            { continue;}
            else if item as char=='\n' {
            fin=i-1;
            }
            else if st==0&&item as char !=' ' {
            st=i;
            } 
    }
    if fin>st { let fnstr:&str=&Str[st..fin];
    let res:String= fnstr.to_string();
    return res
    }
    else {
    let res="worng".to_string();
    return  res
    }

}
fn Find_massage2 (s:&String,index:usize) ->String {//返回index后的一个回车信息例如name,version 
    let mut  st:usize  =0;
    let mut  fin:usize  =0;
    let ch:&str=&s[index..];
    let Str:String =ch.to_string();
    let bytes = Str.as_bytes();
   // for (i, &item) in bytes.iter().enumerate() {
   //     println!("{} {}", i, item as char);
   // }
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

fn write_purl(Name:&String,Version:&String,tag:usize){
   let mut ver=String::new();
   match tag{
       1=>{ver.push_str("20.03");}
       2=>{ver.push_str("20.03sp1");}
       3=>{ver.push_str("20.03sp2");}
       4=>{ver.push_str("20.03sp3");}
       5=>{ver.push_str("20.09");}
       6=>{ver.push_str("21.03");}
       7=>{ver.push_str("21.09");}
       _=>{println!("something wrong");}
   }
   let purl1 = PackageUrl::new("rpm", &*Name)
    .expect("only fails if type is invalid")
    .with_namespace("openeuler")
    .with_version(Version)
    .add_qualifier("arch","arrch64")
    .unwrap()
    .add_qualifier("distro","oe1")
    .unwrap()
    .add_qualifier("version",&ver)
    .unwrap()
    .to_string();
  let purl2 = PackageUrl::new("rpm", &*Name)
    .expect("only fails if type is invalid")
    .with_namespace("openeuler")
    .with_version(Version)
    .add_qualifier("arch","source")
    .unwrap()
    .add_qualifier("distro","oe1")
    .unwrap()
    .add_qualifier("version",&ver)
    .unwrap()
    .to_string();
  let purl3 = PackageUrl::new("rpm", &*Name)
    .expect("only fails if type is invalid")
    .with_namespace("openeuler")
    .with_version(Version)
    .add_qualifier("arch","x86_64")
    .unwrap()
    .add_qualifier("distro","oe1")
    .unwrap()
    .add_qualifier("version",&ver)
    .unwrap()
    .to_string();

   match tag{
        1=>{
            let mut file1 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-20.03-LTS-aarch64.manifest")
                .expect( "cannot open file");
            file1.write_all(&Name.as_bytes())
                .expect("write failed");
            file1.write_all(":  ".as_bytes())
                .expect("write failed");
            file1.write_all(&purl1.as_bytes())
                .expect("write failed");
            file1.write_all("\n".as_bytes())
                .expect("write failed");
            
            let mut file2 = OpenOptions::new().append(true)                
                .open("/root/projects/Purl/purl/openEuler-20.03-LTS-source.manifest")
                .expect( "cannot open file");
            file2.write_all(&Name.as_bytes())
                .expect("write failed");
            file2.write_all(":  ".as_bytes())
                .expect("write failed");
            file2.write_all(&purl2.as_bytes())
                .expect("write failed");
            file2.write_all("\n".as_bytes())
                .expect("write failed");

            
            let mut file3 = OpenOptions::new().append(true)                                
                .open("/root/projects/Purl/purl/openEuler-20.03-LTS-x86_64.manifest")
                .expect( "cannot open file");
            file3.write_all(&Name.as_bytes())
                .expect("write failed");
            file3.write_all(":  ".as_bytes())
                .expect("write failed");
            file3.write_all(&purl3.as_bytes())
                .expect("write failed");
            file3.write_all("\n".as_bytes())
                .expect("write failed");
            
        }
        2=>{
            let mut file1 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-20.03-LTS-sp1-aarch64.manifest")
                .expect( "cannot open file");
            file1.write_all(&Name.as_bytes())
                .expect("write failed");
            file1.write_all(":  ".as_bytes())
                .expect("write failed");
            file1.write_all(&purl1.as_bytes())
                .expect("write failed");
            file1.write_all("\n".as_bytes())
                .expect("write failed");

            let mut file2 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-20.03-LTS-sp1-source.manifest")
                .expect( "cannot open file");
            file2.write_all(&Name.as_bytes())
                .expect("write failed");
            file2.write_all(":  ".as_bytes())
                .expect("write failed");
            file2.write_all(&purl2.as_bytes())
                .expect("write failed");
            file2.write_all("\n".as_bytes())
                .expect("write failed");


            let mut file3 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-20.03-LTS-sp1-x86_64.manifest")
                .expect( "cannot open file");
            file3.write_all(&Name.as_bytes())
                .expect("write failed");
            file3.write_all(":  ".as_bytes())
                .expect("write failed");
            file3.write_all(&purl3.as_bytes())
                .expect("write failed");
            file3.write_all("\n".as_bytes())
                .expect("write failed");

        }
        3=>{
            let mut file1 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-20.03-LTS-sp2-aarch64.manifest")
                .expect( "cannot open file");
            file1.write_all(&Name.as_bytes())
                .expect("write failed");
            file1.write_all(":  ".as_bytes())
                .expect("write failed");
            file1.write_all(&purl1.as_bytes())
                .expect("write failed");
            file1.write_all("\n".as_bytes())
                .expect("write failed");

            let mut file2 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-20.03-LTS-sp2-source.manifest")
                .expect( "cannot open file");
            file2.write_all(&Name.as_bytes())
                .expect("write failed");
            file2.write_all(":  ".as_bytes())
                .expect("write failed");
            file2.write_all(&purl2.as_bytes())
                .expect("write failed");
            file2.write_all("\n".as_bytes())
                .expect("write failed");


            let mut file3 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-20.03-LTS-sp2-x86_64.manifest")
                .expect( "cannot open file");
            file3.write_all(&Name.as_bytes())
                .expect("write failed");
            file3.write_all(":  ".as_bytes())
                .expect("write failed");
            file3.write_all(&purl3.as_bytes())
                .expect("write failed");
            file3.write_all("\n".as_bytes())
                .expect("write failed");

            
        }
        4=>{
            
            let mut file1 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-20.03-LTS-sp3-aarch64.manifest")
                .expect( "cannot open file");
            file1.write_all(&Name.as_bytes())
                .expect("write failed");
            file1.write_all(":  ".as_bytes())
                .expect("write failed");
            file1.write_all(&purl1.as_bytes())
                .expect("write failed");
            file1.write_all("\n".as_bytes())
                .expect("write failed");

            let mut file2 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-20.03-LTS-sp3-source.manifest")
                .expect( "cannot open file");
            file2.write_all(&Name.as_bytes())
                .expect("write failed");
            file2.write_all(":  ".as_bytes())
                .expect("write failed");
            file2.write_all(&purl2.as_bytes())
                .expect("write failed");
            file2.write_all("\n".as_bytes())
                .expect("write failed");


            let mut file3 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-20.03-LTS-sp3-x86_64.manifest")
                .expect( "cannot open file");
            file3.write_all(&Name.as_bytes())
                .expect("write failed");
            file3.write_all(":  ".as_bytes())
                .expect("write failed");
            file3.write_all(&purl3.as_bytes())
                .expect("write failed");
            file3.write_all("\n".as_bytes())
                .expect("write failed");

        }
        5=>{
    
            let mut file1 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-20.09-aarch64.manifest")
                .expect( "cannot open file");
            file1.write_all(&Name.as_bytes())
                .expect("write failed");
            file1.write_all(":  ".as_bytes())
                .expect("write failed");
            file1.write_all(&purl1.as_bytes())
                .expect("write failed");
            file1.write_all("\n".as_bytes())
                .expect("write failed");

            let mut file2 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-20.09-source.manifest")
                .expect( "cannot open file");
            file2.write_all(&Name.as_bytes())
                .expect("write failed");
            file2.write_all(":  ".as_bytes())
                .expect("write failed");
            file2.write_all(&purl2.as_bytes())
                .expect("write failed");
            file2.write_all("\n".as_bytes())
                .expect("write failed");


            let mut file3 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-20.09-x86_64.manifest")
                .expect( "cannot open file");
            file3.write_all(&Name.as_bytes())
                .expect("write failed");
            file3.write_all(":  ".as_bytes())
                .expect("write failed");
            file3.write_all(&purl3.as_bytes())
                .expect("write failed");
            file3.write_all("\n".as_bytes())
                .expect("write failed");

        }
        6=>{

            let mut file1 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-21.03-aarch64.manifest")
                .expect( "cannot open file");
            file1.write_all(&Name.as_bytes())
                .expect("write failed");
            file1.write_all(":  ".as_bytes())
                .expect("write failed");
            file1.write_all(&purl1.as_bytes())
                .expect("write failed");
            file1.write_all("\n".as_bytes())
                .expect("write failed");

            let mut file2 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-21.03-source.manifest")
                .expect( "cannot open file");
            file2.write_all(&Name.as_bytes())
                .expect("write failed");
            file2.write_all(":  ".as_bytes())
                .expect("write failed");
            file2.write_all(&purl2.as_bytes())
                .expect("write failed");
            file2.write_all("\n".as_bytes())
                .expect("write failed");


            let mut file3 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-21.03-x86_64.manifest")
                .expect( "cannot open file");
            file3.write_all(&Name.as_bytes())
                .expect("write failed");
            file3.write_all(":  ".as_bytes())
                .expect("write failed");
            file3.write_all(&purl3.as_bytes())
                .expect("write failed");
            file3.write_all("\n".as_bytes())
                .expect("write failed");

        }
        7=>{
            
            let mut file1 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-21.09-aarch64.manifest")
                .expect( "cannot open file");
            file1.write_all(&Name.as_bytes())
                .expect("write failed");
            file1.write_all(":  ".as_bytes())
                .expect("write failed");
            file1.write_all(&purl1.as_bytes())
                .expect("write failed");
            file1.write_all("\n".as_bytes())
                .expect("write failed");

            let mut file2 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-21.09-source.manifest")
                .expect( "cannot open file");
            file2.write_all(&Name.as_bytes())
                .expect("write failed");
            file2.write_all(":  ".as_bytes())
                .expect("write failed");
            file2.write_all(&purl2.as_bytes())
                .expect("write failed");
            file2.write_all("\n".as_bytes())
                .expect("write failed");


            let mut file3 = OpenOptions::new().append(true)
                .open("/root/projects/Purl/purl/openEuler-21.09-x86_64.manifest")
                .expect( "cannot open file");
            file3.write_all(&Name.as_bytes())
                .expect("write failed");
            file3.write_all(":  ".as_bytes())
                .expect("write failed");
            file3.write_all(&purl3.as_bytes())
                .expect("write failed");
            file3.write_all("\n".as_bytes())
                .expect("write failed");

        }
        
        _=>{println!("something wrong");}

   } 
   

}
fn find_exist_once(Pname:&String,Version:&String)->(bool,bool,String){
    let pname:&str=&Pname[..];
    let version:&str=&Version[..];
    let plen:usize =pname.to_string().chars().count();
    let mut tag:bool = false;
    let mut tag2:bool = true;//判断是否找到可疑串
    let mut ne:String=Default::default();
    match version.find(&pname) {
        Some(index) => {//要判断一下，名字-后面跟的是否为数字

           let jud:&str=&version.to_string()[index+plen+1..];
           let Jud:String=jud.to_string();
           ne=Jud.to_string();
          // println!("{}",Jud[..40].to_string());
           let bytes = Jud.as_bytes();
           for (i, &item) in bytes.iter().enumerate() {
              if item as char>='0'&&item as char<='9'
              {tag=true;}
              break;//循环只执行一次
           }
         //判断一下前面是不是回车
         //  println!("{}",index);
          if index!=0 {
           let jud2:&str=&version.to_string()[index-1..];
           let Jud2:String=jud2.to_string();
           let bytes2 = Jud2.as_bytes();
           for (i, &item) in bytes2.iter().enumerate() {
              if item as char !='\n'
              {tag=false;}
             break;//循环只执行一次
              }
          }
        }
        None => {
            println!("not match name",);
            tag2=false;
        }
    }
    (tag,tag2,ne)
}

fn find_exist(Pname:&String,Version:&String)->bool{//询问一下版本镜像里有没有这个包
    let mut tag:(bool,bool,String)=(false,true,Version.to_string());

    while(tag.1==true){//如果还有可疑项就进行判断
        tag=find_exist_once(&Pname,&tag.2);
        if tag.0 == true{return true;}
       // println!("{}",tag.1);
    }
    tag.0
}

fn match_name(Spec:&String,name1:&String)->String{//匹配spec里的name
             //提取软件名
    let spec:&str=&Spec[..];
    let name:&str=&name1[..];
   // println!("{}",name);
    let mut Name :String=Default::default();     
   // println!("{}",spec);
    match spec.find(&name) {
    Some(index) => {
           //println!("{}", index);
           //println!("开始寻找");
    Name=Find_massage2(&spec.to_string(),index+5);
          // println!("{}",Name);

    }
    None => {
            println!("not match name")
            }
    }

     Name
}
fn match_version(Spec:&String,version1:&String)->String{//匹配spec里的version
    //软件仓版本
    let spec:&str=&Spec[..];
    let version:&str=&version1[..];
    let mut mas :String=Default::default();
    let mut Version :String=Default::default();
    match spec.find(&version) {
    Some(index) => {
           // println!("{}", index);
           //println!("开始寻找");
    mas=Find_massage2(&spec.to_string(),index+8);//mas是返回的信息，先判断是否为全局变量 %{}
   // println!("{}",&mas);
    let m_len=&mas.len();
    let mut tag1:bool=false;
    let mut tag2:bool=false;
    let mut tag3:bool=false;
    let mut tag4:bool=false;//标记是否是全局变量
    let mut tag:(bool,bool)=(true,true);//标记是否可以找到全局变量
    let mut p1:usize=0;
    let mut p2:usize=0;
    for (i, &item) in mas.as_bytes().iter().enumerate() {
         if item as char=='%' {tag1=true;}
         else if item as char=='{'{tag2=true;p1=i}
         else if item as char=='}'{tag3=true;p2=i}
         if tag1==true&&tag2==true&&tag3==true {tag4=true;}
    }
    if tag4==true{//说明是全局变量
        let mas2:&str=&mas[p1+1..p2];
       // println!("{}",&glob_name);
        let mut de_name:String="".to_string();
        de_name.push_str(mas2);
        let de_len=&de_name.to_string().len();
       // println!("{}",&de_name);;
           
        match spec.find(&de_name[..]){
            Some(index)=>
            {
                if p1>0 {
                    Version=mas[..p1-1].to_string();
                }
                Version.push_str(&Find_massage2(&spec.to_string(),index+de_len)[..]);
               // println!("{}",&Version.to_string());
                if p2<m_len-1 {
                    Version.push_str(&mas[p2+1..]);
                 //   println!("{}",&mas[p2+1..].to_string());
                }
               // println!("{}",&Version.to_string());

            }
            None=>{println!("not match de_name");tag.0=false;}
        }
        if tag.0==false {println!("找不到spec中的全局变量");process::exit(1);}

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
fn find_cname(list:&String,cnt:usize)->String{//根据list ，cnt返回该判断的文件名
    let mut cnt_str:String=cnt.to_string();
    cnt_str.push_str(" ");
    let num_str:&str=&cnt_str[..];
    let List:&str =&list[..];
    let mut cname:String=Default::default();
    match List.find(&num_str) {
    Some(index) => {
      cname=Find_massage(&list,index+cnt_str.len()-1);
     }
    None => {
            println!("not match cname")
            }
    }
    cname
}
fn find_path(path:&String)->String{//返回确定的spec文件路径
    let mut Path:String=Default::default();

    for entry in glob(&path).expect("Failed to read glob pattern") {
    match entry {
        Ok(path) =>{ Path=path.display().to_string();},
        Err(e) => println!("{:?}", e),
    }
    }
    Path
}
fn git2_download(name:&String){//git2下载器
     let mut url =String::from("https://gitee.com/src-openeuler/");
     let name_str:&str=&name[..];
     url.push_str(&name_str);
     url.push_str(".git");
     let mut path=String::from("/root/Cangku/");
     path.push_str(&name_str);
     // println!("{}",url);
     // println!("{}",path);
     let repo = match Repository::clone(&url, path) {
     Ok(repo) => repo,
     Err(e) => panic!("failed to clone: {}", e), };
}
fn rmdir(name:&String) {//删除文件夹
    let mut path=String::from("/root/Cangku/");
    let name_str:&str=&name[..];
    path.push_str(&name_str);
    fs::remove_dir_all(path);
}
fn check_version(version:&String){
    let bytes = version.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
    let ch:char =item as char;
    if (ch>='0'&&ch<='9')||(ch=='+')||(ch=='-')||(ch=='.')||('a'<=ch&&ch<='z')||('A'<=ch&&ch<='Z')||(ch=='_') {}
    else
    {
        println!("version存疑");process::exit(1);
        }
    }
}
fn main(){
    let list = fs::read_to_string("/root/projects/Purl/cangkulist.txt").unwrap();//引入仓库清单
    let mut cnt=1;
    while &cnt<=&8408
    {
        let v1 = fs::read_to_string("/root/projects/Purl/version_package/openEuler-20.03-LTS.txt").unwrap();//打开版本包
        let v2 = fs::read_to_string("/root/projects/Purl/version_package/openEuler-20.03-LTS-SP1.txt").unwrap();
        let v3 = fs::read_to_string("/root/projects/Purl/version_package/openEuler-20.03-LTS-SP2.txt").unwrap();
        let v4 = fs::read_to_string("/root/projects/Purl/version_package/openEuler-20.03-LTS-SP3.txt").unwrap();
        let v5 = fs::read_to_string("/root/projects/Purl/version_package/openEuler-20.09.txt").unwrap();
        let v6 = fs::read_to_string("/root/projects/Purl/version_package/openEuler-21.03.txt").unwrap();
        let v7 = fs::read_to_string("/root/projects/Purl/version_package/openEuler-21.03.txt").unwrap();
         //根据名单找到仓库名
         let name:String=find_cname(&list,cnt);
         //判断是否需要生成purl
         let mut tag:bool=false;
         let arr:[String;7]=[v1,v2,v3,v4,v5,v6,v7];
        // println!("{}",find_exist(&name,&arr[6]));
         for index in 0..7 {
         if find_exist(&name,&arr[index]) {tag=true;break;}
         }
         if tag==false {println!("{} no produce",cnt);cnt=cnt+1;continue;}
         //下载代码仓
         println!("正在下载第{}个仓库，仓库名为{}",cnt,&name);
         git2_download(&name);
            
         let name_str:&str=&name[..];
         let mut path=String::from("/root/Cangku/");//构建选择路径
         path.push_str(&name_str);
         path.push_str("/");
         path.push_str("*.spec");
         //println!("{}",path);
         println!("正在生成第{}个仓库的purl，仓库名为{}",cnt,&name);
            //挑选出文件打开路径 
            let Path2:String=find_path(&path);
           // println!("{}",Path2);
            //打开路径
            let contents = fs::read_to_string(&Path2[..]).unwrap();
            let spec :&str =&contents[..];
            //挑选出包版本
            let version :String="Version:".to_string();
            let mut Version :String=Default::default();
            Version=match_version(&contents,&version);
            println!("--{}--",Version);
           // check_version(&"0.9".to_string());
            check_version(&Version);
            for index in 0..7 {
                 if find_exist(&name,&arr[index]) {write_purl(&name,&Version,index+1);}
            }
          //  println!("{}",&name);
          //  println!("{}",find_exist(&name,&arr[1]));
            //删除代码仓
            println!("正在删除第{}个仓库，仓库名为{}",cnt,&name);
            rmdir(&name);       
            println!("生成完毕！{}",cnt);
            cnt=cnt+1;  
    }
    println!("您的purl已全部生成完毕");
}
