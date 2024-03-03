use std::io;

fn main(){

    println!("Welcom to the simple calculator !");
    println!("Choose any action :");
    println!("1. Quadratic equation
2. Sum
3. Difference
4. Product
5. Truncated          ");
    println!("Note : if you wanna exit, push CTRL + C");

let mut choose = String::new();
io::stdin().read_line(&mut choose).expect("Failed to read line");
let choose: i64 = choose.trim().parse().expect("Err. Write a num !");

if choose == 1 {
    quadratic_equation();
} else if choose == 2{
    sum();
} else if choose == 3{
    difference();
} else if choose == 4 {
    product();
} else if choose == 5 {
    truncated();
}

}

fn quadratic_equation(){

    println!("Please, write a first num :");    
    let mut a = String::new();
    io::stdin().read_line(&mut a);
    let a: f64 = a.trim().parse().expect("Err. Write a num !");
                
    println!("Please, write a second num :");
    let mut b = String::new();
    io::stdin().read_line(&mut b);
    let b: f64 = b.trim().parse().expect("Err. Write a num !");          

    println!("Please, write a third num :");  
    let mut c  = String::new();
    io::stdin().read_line(&mut c);
    let c: f64 = c.trim().parse().expect("Err. Write a num !");          

    let d:f64 = b*b - 4.0*a*c;
    
    let x1:f64 = -b + f64::sqrt(d) / 2.0*a;  
    let x2:f64 = -b - f64::sqrt(d) / 2.0*a;  

    println!("The result is :");
    println!("D = {}", d);
    println!("x1 = {}", x1);
    println!("x2 = {}", x2);
    
    main();
}

fn sum(){

    println!("Please, write a first num :");    
    let mut k = String::new();
    io::stdin().read_line(&mut k);
    let k: f64 = k.trim().parse().expect("Err. Write a num !");
                
    println!("Please, write a second num :");
    let mut j = String::new();
    io::stdin().read_line(&mut j);
    let j: f64 = j.trim().parse().expect("Err. Write a num !");          

    println!("Please, write a third num :");  
    let mut v  = String::new();
    io::stdin().read_line(&mut v);
    let v: f64 = v.trim().parse().expect("Err. Write a num !");

    let summa: f64 = k + j + v;
    println!("The result is : {}", summa);

    main();
}

fn difference(){

    println!("Please, write a first num :");    
    let mut l = String::new();
    io::stdin().read_line(&mut l);
    let l: f64 = l.trim().parse().expect("Err. Write a num !");
                
    println!("Please, write a second num :");
    let mut f = String::new();
    io::stdin().read_line(&mut f);
    let f: f64 = f.trim().parse().expect("Err. Write a num !");          

    let differ: f64 = l - f;
    println!("Ther result is : {}", differ);

    main();
}

fn product(){

    println!("Please, write a first num :");    
    let mut u = String::new();
    io::stdin().read_line(&mut u);
    let u: f64 = u.trim().parse().expect("Err. Write a num !");
                
    println!("Please, write a second num :");
    let mut y = String::new();
    io::stdin().read_line(&mut y);
    let y: f64 = y.trim().parse().expect("Err. Write a num !");  

    let prod: f64 = u*y;
    println!("The result is : {}", prod);

    main();
}

fn truncated(){

    println!("Please, write a first num :");    
    let mut l = String::new();
    io::stdin().read_line(&mut l);
    let l: f64 = l.trim().parse().expect("Err. Write a num !");
                
    println!("Please, write a second num :");
    let mut f = String::new();
    io::stdin().read_line(&mut f);
    let f: f64 = f.trim().parse().expect("Err. Write a num !");          

    let differ: f64 = l / f;
    println!("Ther result is : {}", differ);

    main();
}