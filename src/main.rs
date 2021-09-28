/*!
Tools for handling barcode and UMI.
*/

use std::fs;
use std::io::{self, Write};
use std::str;

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
    /// pattern
    #[structopt(long = "pattern")]
    pattern: String,
    /// whitelist
    #[structopt(long = "whitelist")]
    whitelist: String,
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

    let whitelist = barcode::read_one_col(&args.whitelist);
    let pattern_dict = barcode::parse_pattern(&args.pattern);
    let mismatch_dict = barcode::get_mismatch_dict(&whitelist, 1);
    let mut mismatch_dict_list = Vec::new();
    for _ in 0..pattern_dict[&'C'].len() {
        mismatch_dict_list.push(mismatch_dict.clone());
    }

    for result in fastq1 {
        let record1 = result.expect("Error during fastq record parsing");
        let record2 = fastq2.next().unwrap().unwrap();
        let record1_seq = str::from_utf8(record1.seq()).unwrap();
        let pattern_seq = barcode::get_pattern_seq(
            record1_seq, &pattern_dict, 'C');
        let option = barcode::check_seq_mismatch(pattern_seq, &mismatch_dict_list);
        if let Some(corrected_seq) = option {
            let umi = barcode::get_pattern_seq(record1_seq, &pattern_dict, 'U');
            let umi: String = umi.iter().map(|x| *x).collect();
            let read_name = [&corrected_seq, "_", &umi].concat();
            out_fastq2.write(&read_name, None, record2.seq(), record2.qual()).unwrap();
        }
    }

}


