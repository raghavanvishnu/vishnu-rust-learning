
fn main () {

    let name = "Vishnu";
    let age = 43;
    let salary = 260000;

    let mut marks = 76;

    println!("name: {}", name);
    println!("age:{} ", age);
    println!("salary: {}", salary);
    println!("marks: {}", marks);

    marks = 80;

    println!("revised marks: {}", marks);

    let answer = square(25);
    println!("answer:{}", answer);

}

fn square(x:i32)->i32 {

x*x

}


