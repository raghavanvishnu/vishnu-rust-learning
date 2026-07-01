fn main(){
let marks =vec![11,12,11,14,15];
println!("Students Marks analyser");
println!("{:?}",marks);

for mark in &marks{
    println!("{}",mark)
}

let mut total =0;
for mark in &marks{
    total += mark;
    }

    println!("{}", total);
}