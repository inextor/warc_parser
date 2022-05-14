use libflate::gzip::MultiDecoder as GzipReader;
use std::io::BufReader;
use std::io::BufRead;
//use reqwest::blocking::Response;

pub fn parse_reverse_url(url_string:&str)->String
{
    let parts = url_string.split(')');
    let vec:Vec<&str> = parts.collect();
    let firsts = match vec.first() 
    {
        Some(some_str)=>some_str,
        None=> ""
    };

    let parts = firsts.split(',');
    let mut vec:Vec<&str> = parts.collect();
    vec.reverse();
    vec.join(".")
}


pub fn download_gzip_file(url:&str)->Option<Box<dyn BufRead>>
{
    match reqwest::blocking::get(url)
    {
        Ok(response)=>
        {
            let buff_reader =  BufReader::with_capacity(4*1_048_576, response );
            match GzipReader::new( buff_reader )
            {
                Ok(gzip_stream)=>{
                    let buff_reader2 = BufReader::with_capacity(4*1_048_576,gzip_stream);
                    Some( Box::new( buff_reader2 ) )
                },
                Err(error)=>{
                    eprintln!("Ocurrio un error {}",error);
                    None
                }
            }
        },

        Err(error)=>{

            eprintln!("Ocurrio un error {}",error);
            None
        }
    }
}

pub fn u8_tostring(body:&[u8])->String
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

//pub fn available_cpu()->u64
//{
//    match thread::available_parallelism() 
//    {
//        Ok(f)=>{
//            f.get().try_into().unwrap()
//        },
//        Err(_)=>{
//            0
//        }
//    }
//}
