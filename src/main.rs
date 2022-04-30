//extern crate warc;
use std::process;
use std::env;
use warc::WarcReader;
use lazy_static::lazy_static;
use regex::Regex;
//use html_parser::Dom;

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 
    {
        println!("Usage warc_parser warc_file.warc.gz");
        process::exit(1);
    }

    let filepath = args.get(1).expect("Usage warc_parser warc_file.warc.gz");


    //let file = WarcReader::from_path("warc_example.warc")?;
    let file = WarcReader::from_path_gzip( filepath ).expect("Not a warc valid format");
    lazy_static! {
        //let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        //static ref RE: Regex = Regex::new(r"(?x)
        //    (?P<login>[^@\s\#/\{\}<>,\u0022!:;\[\]'\(\)]+@
        //    #([[:word:]]+\.)+ Este es un comentario
        //    ([^@\s\#/\{\}<>,\u0022!:;\[\]'\(\)%]+\.)+
        //    [[:word:]]{2,28})
        //    ").unwrap();
        //
        //
        static ref RE: Regex = Regex::new(r"(?xm)
            (?P<svg><svg>.+</svg>)
            ").unwrap();

    }


    for record in file.iter_records() {
        match record {
            Err(_err) =>{},
            Ok(record) => {
                let warkid = match record.header(warc::WarcHeader::TargetURI).map(|s| s.to_string()) {
                    Some(v)=>v,
                    None=> "".to_string()
                };

                let s:String = get_string(record.body());

                for caps in RE.captures_iter(&s.to_string()){

                    println!("{}\t{}",&caps["svg"],warkid);
                }
           }
        }
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

