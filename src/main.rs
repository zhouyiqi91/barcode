/*!
Tools for handling barcode and UMI.
*/

use std::fs;
use std::io::{self, Write};

use log::{info, warn};
use simplelog::*;
use structopt::StructOpt;
use anyhow::{Context, Result};
use bio::io::fastq;

use barcode;

/// cli
#[derive(StructOpt)]
struct Cli {
    /// fastq 1
    #[structopt(long = "fq1")]
    fq1: String,
    /// fastq 2
    #[structopt(long = "fq2")]
    fq2: String,
}


fn main() {

    // logger
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), fs::File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    //IO
    let out_fq2_file = "out_2.fq";
    let mut out_fastq2 = fastq::Writer::to_file(out_fq2_file).unwrap();

    let args = Cli::from_args();

    let fastq1 = fastq::Reader::from_file(&args.fq1).unwrap().records();
    let mut fastq2 = fastq::Reader::from_file(&args.fq2).unwrap().records();

    for result in fastq1 {
        let record1 = result.expect("Error during fastq record parsing");
        let record2 = fastq2.next().unwrap().unwrap();

        out_fastq2.write_record(&record2).unwrap();
    }

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

