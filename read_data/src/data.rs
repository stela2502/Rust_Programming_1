
use ndarray::ArrayBase;
use ndarray::Dim;
use ndarray::Array;
use std::io::BufRead;


#[derive(Debug)]
pub struct Data{
	pub rows:usize, // the amount of rows
	pub cols:usize, // the amount of cols
	pub rownames: Vec::<String>, //rge rownames of the data - we will cluster them
	pub data: ArrayBase<ndarray::OwnedRepr<f64>, Dim<[usize; 2]>>,
}


impl Data {

	pub fn new( rows:usize, cols:usize, data: Vec::<f64>, rownames: Vec::<String> ) -> Self {
		//let ret = &data as &[f64]; 
		let ret = Array::from_iter(&mut data.into_iter());
		let data = ret.into_shape([rows, cols]).unwrap();

		Self {
			rows, 
			cols,
			rownames,
			data,
		}
	}

	pub fn read_file( file:&std::string::String, sep:char ) -> Self {

	    let mut cols = 0;
	    let mut rows = 0;
	    // get the data dimensions
	    {
	        let fi = std::fs::File::open( file ).unwrap();
	        let reader = std::io::BufReader::new(fi);

	        for line in reader.lines() {
	            match line {
	                Ok(line) => {
	                    cols = line.split( sep ).count() -1;
	                    rows +=1;
	                },
	                Err(err) => {
	                    panic!("Unexpected error reading the csv file: {err:?}");
	                }
	            }
	        }
	    }
	    rows -=1;
	    println!("I got {rows} rows and {cols} cols in this data"  );

	    let mut arr  = Vec::<f64>::with_capacity( cols * rows );
	    let mut names = Vec::<String>::with_capacity( cols );

	    let fi = std::fs::File::open( file ).unwrap();
	    let reader = std::io::BufReader::new(fi);

	    let mut header = true;

	    for line in reader.lines() {
	        if header{
	            // just drop the header
	            header = false;
	            continue;
	        }
	        match line {
	            Ok(line) => {
	                header =true;
	                for mut val in line.split( sep ).collect::<Vec<&str>>(){
	                    if header{
	                        names.push( val.to_string() );
	                        header = false;
	                    }else {
	                    	val = val.trim();
	                        let v = match val.parse::<f64>() {
	                            Ok( v ) => v,
	                            Err(_err) => {
	                                match val.parse::<usize>(){
	                                    Ok(v) =>  { 
	                                        v as f64
	                                    },
	                                    Err(err) => {
	                                        eprintln!("I could not parse '{val}' to usize or f64 {err:?}");
	                                        0.0
	                                    },
	                                }
	                            },
	                        };
	                        arr.push( v );
	                    }
	                }
	            },
	            Err(err) => {
	                panic!("Unexpected error reading the csv file: {err:?}");
	            }
	        }
	    }

	    Self::new( rows, cols, arr,  names )
	}

	pub fn print ( &self ){
		println!("{}", self.data);
	}

}

#[cfg(test)]
mod tests {

    use crate::data::Data;

     #[test]
    fn check_read() {

    	let data = Data::read_file( &"testData/Spellman_Yeast_Cell_Cycle.tsv".to_string(), '\t' );

    	assert_eq!( data.data[[0,0]], -0.35 );
    	assert_eq!( data.data[[1,1]], -1.98 );
    }

}



