use libflate::gzip::MultiDecoder as GzipReader;
use std::io::BufReader;
use reqwest::blocking::Response;

fn parse_url(url_string:String)->String
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

pub fn download_gzip_file(url:&str)->Result<GzipReader<BufReader<Response>>,&str>
{
    match reqwest::blocking::get(url)
    {
        Ok(response)=>
        {
            match GzipReader::new(BufReader::with_capacity(4*1_048_576, response ))
            {
                Ok(gzip_stream)=>{
                    Ok(gzip_stream)
                }
                ,Err(gzip_err)=>
                {
                    eprintln!("Error {}",gzip_err);
                    Err("")
                }
            }
        },
        Err(net_err)=>
        { 
            eprintln!("Error {}",net_err);
            Err("")
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
