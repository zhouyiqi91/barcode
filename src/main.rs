/**
command line 
*/

use std::fs;
use std::io::{self, Write};

use log::{info, warn};
use simplelog::*;
use structopt::StructOpt;
use anyhow::{Context, Result};

/// cli
#[derive(StructOpt)]
struct Cli {
    /// fastq 1
    #[structopt(long = "fq1")]
    fq1: String,
    /// fastq2
    #[structopt(long = "fq2")]
    fq2: String,
}


fn main() -> Result<()>{

    // logger
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), fs::File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    //IO
    let out_fq1_file = "out_1.fq";
    let out_fq1 = fs::File::create(out_fq1_file).unwrap();
    let mut handle = io::BufWriter::new(out_fq1);
    let args = Cli::from_args();

    info!("read fastq1");
    warn!("warn you!!!!!");
    let fq1 = fs::read_to_string(&args.fq1)
        .with_context(|| format!("could not read file `{}`", &args.fq1))?;
    write_fastq(fq1, &mut handle);
    
    Ok(())
}


fn write_fastq<W>(content: String, mut writer: W) 
where W: std::io::Write
{
    for line in content.lines() {
        writer.write_all(line.as_bytes()).unwrap();
        writer.write(&['\n' as u8]).unwrap();
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_write_fastq() {
        let mut writer = Vec::new();
        let contents = String::from("contents");
        write_fastq(contents, &mut writer);
        println!("{:?}", writer);
        
        //let output = String::from_utf8(writer).unwrap();
        assert_eq!(writer, b"contents\n");
    }

    #[test]
    fn vec_test(){
        let mut writer = Vec::new();
        writer.push('a' as u8);
        assert_eq!(writer, b"a");
    }
}

