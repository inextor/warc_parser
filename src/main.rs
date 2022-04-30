//extern crate warc;
use std::{str,process, borrow::Borrow, clone};
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
            (?P<login>[^@\s\#/\{\}<>,\u0022!:;\[\]'\(\)]+@
            #([[:word:]]+\.)+ Este es un comentario
            ([^@\s\#/\{\}<>,\u0022!:;\[\]'\(\)%]+\.)+
            [[:word:]]{2,28})
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
                //

                let s:String = get_string(record.body());

                for caps in RE.captures_iter(&s.to_string()){
                    println!("{}\t{}",&caps["login"],warkid);
                }

                if counter > 71862 
                {
                    println!("{} ",warkid);
                    println!("the shits ends {} ", s);
                    process::exit(1);
                }

           }
        }

        //counter += 1;

        //if counter == 3 
        //{
        //    process::exit(1);
        //}
    }
}

fn get_string(body:&[u8])->String
{
    let s = match String::from_utf8(body.to_vec())
    {
        Ok(ss)=>ss,
        Err(_)=> String::from("")
    };

    if !s.is_empty()
    {
        return s;
    };

    return String::from_utf8_lossy( body ).to_string();
}

