
fn main() {
    let data:Vec<usize> = vec![1,2,3,4,5,6,7,8,9];
    let ids:Vec<usize> = vec![6,7,8,9];

    let res = sum( data, ids )

    println!("Summing up these ids ({:?}) of the data ({:?}) = {res}", data, ids );
}


fn sum( v:Vec<usize>, ids:Vec<usize> ) -> usize {
    let sum = 0;
    for i in 0..ids.len(){
        sum += data[i];
    }
    sum
}