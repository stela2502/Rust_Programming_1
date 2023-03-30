use clap::Parser;

use std::path::PathBuf;
use std::fs;
use std::time::SystemTime;

use read_data::data::Data;


/// Run a simulated anealing clustering over the rows of the provided data. 
/// The software is a demo project for the Lund Stem Cell Center - Bioinformatics Rust course.

#[derive(Parser)]
#[clap(version = "1.0.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Opts {
    /// the data (text file)
    #[clap(default_value= "testData/Spellman_Yeast_Cell_Cycle.tsv",short, long)]
    data: String,
    /// the column separator for the file
    #[clap(default_value= "\\t",short, long)]
    sep: String,
}

fn main() {
    let now = SystemTime::now();
    
    let opts: Opts = Opts::parse();
    let mut sep = '\t';
    if &opts.sep != "\\t"{
        println!("I set sep to {}", opts.sep );
        sep = opts.sep.chars().next().unwrap(); 
    }

    let data = Data::read_file( &opts.data , sep );

    data.print();

    match now.elapsed() {
        Ok(elapsed) => {
            let mut milli = elapsed.as_millis();

            let mil = milli % 1000;
            milli= (milli - mil) /1000;

            let sec = milli % 60;
            milli= (milli -sec) /60;

            let min = milli % 60;
            milli= (milli -min) /60;

            println!("finished in {milli} h {min} min {sec} sec {mil} milli sec");
        },
        Err(e) => {println!("Error: {e:?}");}
    }
}
