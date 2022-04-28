//extern crate warc;
use std::{str, process};
use warc::WarcReader;
//use html_parser::Dom;

fn main(){
    //let file = WarcReader::from_path("warc_example.warc")?;
    let file = WarcReader::from_path_gzip("/home/nextor/Projects/CommonCrawl/file1.warc.gz").unwrap();
    let mut counter = 0;

    for record in file.iter_records() {
        match record {
            Err(_err) =>{},
            Ok(record) => {

                let mut length = 0;
                match record.header( warc::WarcHeader::ContentLength).map(|s| s.to_string()){
                    Some(v)=>{
                        println!("Total Length {}",v);
                        length=v.parse::<i32>().unwrap();
                    },
                    None=>{}
                };

                println!("Length from contentLength {}",record.content_length());


                //mut iter = body.split_inclusive(|num| num % length );

                if let Ok(s) = str::from_utf8(record.body())
                {
                    let slength = s.len();

                    match s.find("\n\n") 
                    {
                        Some(pos)=>{
                            
                            if let Some(real_body) = s.get(pos+2..slength)
                            {
                                println!("Real body is {}", real_body);
                            }
                        },
                        None=>{}
                    }

                    if slength == 0
                    {
                        println!("length {} diff {}", slength, length);
                    }
                    else
                    {
                        println!("length {}", slength);
                    }
                }
               

                match record.header( warc::WarcHeader::IdentifiedPayloadType).map(|s| s.to_string()){
                    Some(v)=>println!("idtype {}",v),
                    None=>{}
                };



                
                //match str::from_utf8(buff){
                //    Ok(s) =>{ println!(" {} ", s );}
                //    Err(_e)=>{}
                //};
            }
        }

        counter += 1;

        if counter == 3 
        {
            process::exit(1);
        }
    }
}

