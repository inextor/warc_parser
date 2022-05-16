use std::collections::HashMap;
use std::io::BufRead;
//extern crate warc;
use std::process;
use std::env;
use std::io::BufReader;
use warc::WarcReader;
use lazy_static::lazy_static;
use regex::Regex;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

mod utils;

fn main(){

    let args: Vec<String> = env::args().collect();

    if args.len() < 2
    {
        eprintln!("Usage warc_parser 3");
        process::exit(1);
    }
    let thread_str = args.get(1).expect("Usage warc_parser warc_file.warc.gz");
    eprintln!("Running with {} ",thread_str);

    //parse_path_files("http://127.0.0.1/cdx-00000.gz");
    let max_threads = thread_str.parse::<u64>().unwrap();
    spawn_threads( max_threads );
    //let r = utils::download_gzip_file2222("http://127.0.0.1/cdx-00000.gz");
    //eprintln!("FOOO {}", r.unwrap());
    //xxxxx();
}

//Use active_count instead  of spawning a every loop
//https://docs.rs/threadpool/latest/threadpool/struct.ThreadPool.html#method.active_count

fn spawn_threads(max_threads:u64)
{
    let cpus = num_cpus::get();

    eprintln!("Number of cpus {}",cpus);


    let mut counter = 1;

    let pool = ThreadPool::new( max_threads.try_into().unwrap() ); 

    loop{

        let active = pool.active_count();
        let u64_active:u64  = active.try_into().unwrap();
        let available = max_threads-u64_active; 

        if available > 0 
        {
            eprintln!("threads active {} max_available Threads {}, available {}",active, max_threads, available);
        }

        if available > 0 
        {
            counter+=1;
            let s = format!("http://127.0.0.1/warcs/filename{}.warc.gz",counter);
            pool.execute( move ||{
                eprintln!("Running inside thread {}", s);
                let v = parse_url_warc_gzip(&s);
                eprintln!("Number of emails {}",v.len());
            });

        }

        thread::sleep(Duration::from_millis(400));

        if counter >= 39 
        {
            break;
        }
    }

    pool.join();
    //xx();
}

// parse_path_files("http://127.0.0.1/cdx-00000.gz");

fn parse_path_files(url:&str)
{
    let mut counter = 0;
    let mut hashmap:HashMap<String,u64> = HashMap::with_capacity(200_000); 
    match utils::download_gzip_file( url ) 
    {
        Some(mut x)=>{
            let mut buffer_str = String::new();
            while let Ok(size) = x.read_line(&mut buffer_str) 
            { 
                let domain = utils::parse_reverse_url( &buffer_str );
                 
                hashmap.entry(domain).or_insert_with(|| {                      
                    counter += 1;                      
                    counter                          
                });

                //if !hashmap.contains_key(&domain)
                //{
                //    counter += 1;
                //    hashmap.insert(domain,counter);        
                //}

                buffer_str.clear();

                if size == 0 
                {
                    break;
                }
            }
        },
        None=>{eprintln!("An error occour");}
    }
    for (domain,id) in hashmap.iter()
    {
        println!("{} {}",id,domain);
    }
}

fn parse_url_warc_gzip(url:&str)->Vec<String>
{
    lazy_static! {
        //let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        static ref RE: Regex = Regex::new(r"(?x)
            (?P<a>[^@\s\#/\{\}<>,\u0022!:;\[\]'\(\)]+@
            #([[:word:]]+\.)+ Este es un comentario
            ([^@\s\#/\{\}<>,\u0022!:;\[\]'\(\)%]+\.)+
            [[:word:]]{2,10})
            ").unwrap();
    }

    let mut v:Vec<String> = Vec::new();

    match utils::download_gzip_file( url ) 
    {
        Some(response)=>{
            let x = WarcReader::new(BufReader::new(response));
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
                        let s:String = utils::u8_tostring(record.body());

                        for caps in RE.captures_iter(&s.to_string()){
                            let s_line = format!("{}\t{}",&caps["a"],url);
                            //println!("{}",s_line );
                            v.push(s_line);
                        }
                   }
                }
            }
            v
        },
        None=>{v}
    }
}

