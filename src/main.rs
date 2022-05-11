//extern crate warc;
use std::process;
use std::env;
use std::io::BufReader;
use warc::WarcReader;
//use lazy_static::lazy_static;
//use regex::Regex;
//use html_parser::Dom;
//use reqwest::blocking;

use libflate::gzip::MultiDecoder as GzipReader;

fn main(){

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 
    {
        println!("Usage warc_parser warc_file.warc.gz");
        process::exit(1);
    }

    let url = args.get(1).expect("Usage warc_parser warc_file.warc.gz");
    download_file( url );

}


fn download_file(url:&str)
{

    /*
    if let Ok( response ) = reqwest::blocking::get("http://127.0.0.1/file1.warc.gz")
    {
        if let Ok( gzip_stream ) = GzipReader::new(BufReader::with_capacity(4*1_048_576, response ))
        {
            let x = WarcReader::new(BufReader::new(gzip_stream));

            for record in x.iter_records() {
                match record {
                    Err(_err) =>{},
                    Ok(record) => {
                        let warkid = match record.header(warc::WarcHeader::TargetURI).map(|s| s.to_string()) {
                            Some(v)=>v,
                            None=> "".to_string()
                        };

                        println!("Id is {}", warkid);
                   }
                }
            }
        }
    }
    */

    match reqwest::blocking::get(url)
    {
        Ok(response)=>{  
            match GzipReader::new(BufReader::with_capacity(4*1_048_576, response ))
            {   
                Ok(gzip_stream)=>{
                    let x = WarcReader::new(BufReader::new(gzip_stream));
                    for record in x.iter_records() {
                        match record {
                            Err(err_records) =>{
                                println!("Error on reading records {}",err_records);
                            },
                            Ok(record) => {
                                let warkid = match record.header(warc::WarcHeader::TargetURI).map(|s| s.to_string()) {
                                    Some(v)=>v,
                                    None=> "".to_string()
                                };

                                let record_id = match record.header(warc::WarcHeader::RecordID).map(|s| s.to_string()) {
                                    Some(v)=>v,
                                    None=> "".to_string()
                                };

                                let warc_type = match record.header(warc::WarcHeader::WarcType).map(|s| s.to_string()) {
                                    Some(v)=>v,
                                    None=> "".to_string()
                                };

                                if warc_type.eq("response") 
                                {
                                    println!("Id is {} {} {}",warc_type,record_id, warkid);
                                }
                           }
                        }
                    }
                },
                Err(error2)=>{
                    println!("Error stream gzip {}",error2);
                }
            }
        },
        Err(err)=>{
            println!("error {} ",err);
        }
    }
}





//    for line in BufReader::new(body).lines() {
//    	println!("{}", line.unwrap());
//    }
//
//    match 
//    {
//        Ok(mut response)=>
//        {
//            if response.status() == reqwest::StatusCode::OK {
//                let reader = BufReader::new(stream); // moved its value
//            }
//            else
//            {
//                println!("Response was not 200 OK");
//            }
//        }
//        err(_)=>println!("Could not make the request!");
//
//    }
//
//
//    //let file = WarcReader::from_path("warc_example.warc")?;
//    let file = WarcReader::from_path_gzip( filepath ).expect("Not a warc valid format");
//    lazy_static! {
//        //let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
//        static ref RE: Regex = Regex::new(r"(?x)
//            (?P<login>[^@\s\#/\{\}<>,\u0022!:;\[\]'\(\)]+@
//            #([[:word:]]+\.)+ Este es un comentario
//            ([^@\s\#/\{\}<>,\u0022!:;\[\]'\(\)%]+\.)+
//            [[:word:]]{2,28})
//            ").unwrap();
//        //
//        //
//        //static ref RE: Regex = Regex::new(r"(?xm)
//        //    (?P<svg><svg>.+</svg>)
//        //    ").unwrap();
//
//    }
//
//
//    for record in file.iter_records() {
//        match record {
//            Err(_err) =>{},
//            Ok(record) => {
//                let warkid = match record.header(warc::WarcHeader::TargetURI).map(|s| s.to_string()) {
//                    Some(v)=>v,
//                    None=> "".to_string()
//                };
//
//                let s:String = get_string(record.body());
//
//                for caps in RE.captures_iter(&s.to_string()){
//
//                    println!("{}\t{}",&caps["login"],warkid);
//                }
//           }
//        }
//        
//    }
//}

#[warn(dead_code)]
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

