//extern crate warc;
use std::{str,process, borrow::Borrow};
use warc::WarcReader;
use lazy_static::lazy_static;
use regex::Regex;
use std::ffi::OsString;
//use html_parser::Dom;

fn main(){
    //let file = WarcReader::from_path("warc_example.warc")?;
    let file = WarcReader::from_path_gzip("/home/nextor/Projects/CommonCrawl/file1.warc.gz").unwrap();
    lazy_static! {
        //let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        static ref RE: Regex = Regex::new(r"(?x)
            (?P<login>[^@\s\#/\{\}<>,\u0022!:;\[\]']+@
            ([[:word:]]+\.)+
            [[:word:]]+)
            ").unwrap();
    }

    let mut counter:i64 = 0;
    let mut length:usize = 0;

    for record in file.iter_records() {
        match record {
            Err(_err) =>{},
            Ok(record) => {

                counter += 1;
                match record.header( warc::WarcHeader::ContentLength).map(|s| s.to_string()){
                    Some(v)=>{
                        length=v.parse::<usize>().unwrap();
                    },
                    None=>{ length = 0}
                };

                let warkid = match record.header(warc::WarcHeader::TargetURI).map(|s| s.to_string()) {
                    Some(v)=>v,
                    None=> "".to_string()
                };

                //println!("Length from contentLength {}",record.content_length());


                //mut iter = body.split_inclusive(|num| num % length );


                let mut s = match str::from_utf8(record.body())
                {
                    Ok(ss)=>ss,
                    Err(_)=> ""
                };


                let ss = String::from_utf8_lossy(record.body() ).to_string();

                


                if let Ok(s) = str::from_utf8(record.body())
                {

                    if length == 0 
                    {
                        length = s.len();
                    };

                    //match s.find("\n\n") 
                    //{
                    //    Some(pos)=>{

                            //if let Some(real_body) = s.get(pos+2..length)
                            //{

                                if warkid == "http://0806690000.co.kr/news/article.html?no=267722" 
                                {
                                    println!("{} ",warkid);
                                    println!("the shits ends {} ", s );
                                    process::exit(1);
                                }

                                //println!("Real body is {}", real_body);
                                //
                                for caps in RE.captures_iter(s){
                                    println!("{}\t{}",&caps["login"],warkid);
                                }

                                if counter > 71862 
                                {
                                    println!("{} ",warkid);
                                    println!("the shits ends {} ", s);
                                    process::exit(1);
                                }
                            //}
                    //    },
                    //    None=>{}
                    //}
                }

                //match record.header( warc::WarcHeader::IdentifiedPayloadType).map(|s| s.to_string()){
                //    Some(v)=>println!("idtype {}",v),
                //    None=>{}
                //};
            }
        }

        //counter += 1;

        //if counter == 3 
        //{
        //    process::exit(1);
        //}
    }
}

