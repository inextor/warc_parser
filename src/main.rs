//extern crate warc;
use std::process;
use std::env;
use std::io::BufReader;
use warc::WarcReader;
use lazy_static::lazy_static;
use regex::Regex;
//use html_parser::Dom;
//use reqwest::blocking;

use libflate::gzip::MultiDecoder as GzipReader;

fn main(){

    let args: Vec<String> = env::args().collect();

    if args.len() < 2
    {
        eprintln!("Usage warc_parser warc_file.warc.gz");
        process::exit(1);
    }

    let url = args.get(1).expect("Usage warc_parser warc_file.warc.gz");
    download_file( url );

}


fn download_file(url:&str)
{
    eprintln!("Downloading");

    lazy_static! {
        //let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        static ref RE: Regex = Regex::new(r"(?x)
            (?P<a>[^@\s\#/\{\}<>,\u0022!:;\[\]'\(\)]+@
            #([[:word:]]+\.)+ Este es un comentario
            ([^@\s\#/\{\}<>,\u0022!:;\[\]'\(\)%]+\.)+
            [[:word:]]{2,10})
            ").unwrap();
    }


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
                                eprintln!("Error on reading records {}",err_records);
                            },
                            Ok(record) => {
                                let warc_type = match record.header(warc::WarcHeader::WarcType).map(|s| s.to_string()) {
                                    Some(v)=>v,
                                    None=> "".to_string()
                                };

                                if warc_type.ne("response")
                                {
                                    continue;
                                }

                                let url = match record.header(warc::WarcHeader::TargetURI).map(|s| s.to_string()) {
                                    Some(v)=>v,
                                    None=> "".to_string()
                                };
                                let s:String = get_string(record.body());

                                for caps in RE.captures_iter(&s.to_string()){
                                    println!("{}\t{}",&caps["a"],url);
                                }
                           }
                        }
                    }
                },
                Err(error2)=>{
                    eprintln!("Error stream gzip {}",error2);
                }
            }
        },
        Err(err)=>{
            eprintln!("error {} ",err);
        }
    }
    eprintln!("Finished");
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
