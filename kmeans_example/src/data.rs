
use ndarray::ArrayBase;
use ndarray::Dim;
use ndarray::Array;
use std::io::BufRead;
use ndarray::prelude::*;
use ndarray::ViewRepr;
//use std::collections::BTreeMap;
use rand::Rng;
use ndarray::Axis;
use std::fs::File;
use std::error::Error;
use ndarray::OwnedRepr;
use csv::WriterBuilder;


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
		let ret = Array::from_iter(&mut data.iter().cloned());
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
	    eprintln!("I got {rows} rows and {cols} cols in this data"  );

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

	fn calculate_centroids(&self, ids:&Vec<usize>, k:usize ) -> ArrayBase<ndarray::OwnedRepr<f64>, Dim<[usize; 2]>> {

		let mut centroids = Vec::<f64>::with_capacity(k * self.data.len_of(Axis(1)));
		for cluster_id in 0..k{
			let indices = ids.iter().enumerate().filter(|(_, id)| **id == cluster_id).map(|(i, _)| i).collect::<Vec<_>>();
	   		//println!("The indices I want to select {indices:?}");
	   		let points = self.data.select(Axis(0), &indices);
	   		//println!("This is the points for the centroid {cluster_id}: {points:?}");
	   		for i in 0..points.len_of(Axis(1)){
	   			centroids.push( points.column(i).sum() /  points.len_of(Axis(0)) as f64);
	   		}
	   		//println!("Axis(0) {}, Axis(1) {}: and that are the centroids I calczulated: {},{},{}",points.len_of(Axis(0)),points.len_of(Axis(1)), centroids[cluster_id *3], centroids[cluster_id*3+1], centroids[ cluster_id*3+2 ] );
		}
		//print!(".");
		Array::from_iter(&mut centroids.iter().cloned()).into_shape([k, self.data.len_of(Axis(1)) ]).unwrap()
	}

	fn assign_labels( &self, centroids: &ArrayBase<ndarray::OwnedRepr<f64>, Dim<[usize; 2]>> ) -> Vec<usize>{

		let mut new_labels = Vec::<usize>::with_capacity( self.data.len_of(Axis(0)) as usize) ;

		let mut distances:Vec<f64> = vec![0.0; centroids.len_of(Axis(0)) as usize];
		for data_id in 0..self.data.len_of(Axis(0)){
			for centroid_id in 0..centroids.len_of(Axis(0)){
				distances[centroid_id] =  Data::euclidean_distance(self.data.index_axis(Axis(0), data_id), centroids.index_axis(Axis(0), centroid_id ));
			}

			let mut min = f64::MAX;
			let mut id = 0;
			for centroid_id in 0..centroids.len_of(Axis(0)){
				if min > distances[centroid_id]{
					min = distances[centroid_id];
					id = centroid_id;
				}
			}
			
			new_labels.push(id);
		}
		new_labels
	}


	pub fn kmeans(&self, k: usize, max_it: usize ) ->Vec<usize> {
	    let n = self.data.len_of(Axis(0)) as usize;

	    let mut rng = rand::thread_rng();

	    let mut centroids = Array::zeros((k, self.data.len_of(Axis(1))));
    	
    	for i in 0..k {
        	let sample_idx = rng.gen_range(0..n);
        	centroids.row_mut(i).assign(&self.data.row(sample_idx));
    	}

	    let mut labels:Vec<usize> = self.assign_labels(&centroids);

		//println!("These are my new labels: {:?}", labels);
	    let mut it = 0;

	    loop {
	    	centroids = self.calculate_centroids(&labels, k);
	        let new_labels = self.assign_labels(&centroids);

	        if new_labels == labels || it == max_it{
	        	eprintln!("finished after {it} iterations");
	            break;
	        }
	        it +=1;

	        labels = new_labels;
	        
	        
	        //println!("My centroids:\n({centroids:?})")
	    }
	    match Data::to_csv( &centroids, format!("testData/centroid.csv"), b','){
	    	Ok(_) => (),
	    	Err(e) => {
	    		eprintln!("The centroid data could not be written to file testData/centroid.csv: {e:?}");
	    	}
	    }
	    labels
	}


	fn min ( data: &ArrayBase<ViewRepr<&mut f64>, Dim<[usize; 1]>> ) -> f64 {
		let mut min:f64 = f64::MAX;
		for val in data{
			if val < &min {
				min = *val;
			}
		}
		min
	}

	fn max ( data: &ArrayBase<ViewRepr<&mut f64>, Dim<[usize; 1]>> ) -> f64 {
		let mut min:f64 = f64::MIN;
		for val in data{
			if val > &min {
				min = *val;
			}
		}
		min
	}
	pub fn scale ( &mut self ){
		for mut row in self.data.rows_mut() {
			row -= Self::min( &row );
			row /= Self::max( &row );
		}
	}



	fn euclidean_distance(p1: ArrayView1<f64>, p2: ArrayView1<f64>) -> f64 {
		(p1.iter().zip(p2.iter()).map(|(x, y)| (x - y).powf(2.0)).sum::<f64>()).sqrt()
	}

	pub fn print ( &self ){
		println!("{}", self.data);
	}

	pub fn to_csv( data: &ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, file:String, sep:u8 ) -> Result<(), Box<dyn Error>> {

	    // create a file for writing
	    let file = File::create( file )?;

	    // create a CSV writer
	    let mut writer = WriterBuilder::new()
	        .delimiter(sep)
	        .quote_style(csv::QuoteStyle::Never)
	        .flexible(true)
	        .from_writer(file);

	    // write each row of the array as a CSV record
	    for row in data.rows() {
	        writer.write_record(row.iter().map(|x| x.to_string()))?;
	    }

	    writer.flush()?;
	    Ok(())
	}

}

#[cfg(test)]
mod tests {

    use crate::data::Data;
    use ndarray::Axis;
    use ndarray::s;

    #[test]
    fn check_assign_labels() {
        let data = Data::read_file( &"testData/CellexalVR_TestData_tsne.csv".to_string(), ',' );
        let centroids = data.data.slice(s![..10, ..]).to_owned();
        let kmeans = data.assign_labels( &centroids );
        let mut min = usize::MAX;
    	let mut max = usize::MIN;
    	for k in &kmeans {
    		if min > *k{
    			min = *k;
    		}
    		if max < *k{
    			max = *k;
    		}
    	}
        assert_eq!( [min, max], [0,9] );
    }

     #[test]
    fn check_kmeans() {

    	let mut data = Data::read_file( &"testData/CellexalVR_TestData_tsne.csv".to_string(), ',' );
    	data.scale();
    	let kmeans = data.kmeans( 15, 15000 );
    	assert_eq!( kmeans.len(), data.data.len_of(Axis(0)) );

    	//let exp:Vec<usize> = vec![0; data.data.len_of(Axis(0)) ];

    	let mut min = usize::MAX;
    	let mut max = usize::MIN;
    	for k in kmeans {
    		if min > k{
    			min = k;
    		}
    		if max < k{
    			max = k;
    		}
    	}
    	assert_eq!( [min , max ], [0, 14] );
    }

}



