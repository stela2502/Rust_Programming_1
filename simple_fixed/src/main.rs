
fn main() {
    let data:Vec<usize> = vec![1,2,3,4,5,6,7,8,9];
    let ids:Vec<usize> = vec![5,6,7,8];

    let res = sum( &data, &ids );

    println!("Summing up these ids ({:?}) of the data ({:?}) = {res}", ids, data );
}


fn sum( v:&Vec<usize>, ids:&Vec<usize> ) -> usize {
    let mut sum = 0;
    for i in 0..ids.len(){
        sum += v[ids[i]];
    }
    sum
}


#[cfg(test)]
mod tests {
    use crate::sum;
    #[test]
    fn check_sum() {
      
      let data:Vec<usize> = vec![1,2,3,4,5,6,7,8,9];
      let ids:Vec<usize> = vec![5,6,7,8];
      
    	assert_eq!( sum( &data, &ids ), 30 );
    }
}