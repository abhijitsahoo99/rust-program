// fn main() {
//    let mut x = 5;
//    println!("The value of x is : {x}");
//    x = 6;
//    println!("The value of x is : {x}");
// }

// Const is used in the global scopre where as let is used in the functions.  
// Const is always immutable and can never be altered where as let is by default immutable and can be altered using mut after let.
// const used data type annotations while declaring variable where as let does not.
// const is declared using UPPER CASE AND IF IT IS A SENTENCE THEN UNDERSCORE BETWEEN WORDS where as no such hard and fasten rule for let.
// Const assign values to the variable during compilation time itself where as let doesn't and does it directly in runtime.

// SHADOWING

// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// We are using let keyword again here. Online in normal immutable we just use the variable which retakes to error. To fix it we use out after let. 
// Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable. 

// DIFFERENCE IN SHADOWING AND MUT 
fn main() {
    let spaces = "   ";
    let spaces = zspaces.len();
    // println!("{}", zspaces);
}

let mut spaces = "   ";
spaces = spaces.len();
// error