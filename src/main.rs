fn main() {
    print_menu();
    println!("\n Please make a selection");
    
    // std::io::stdin().read_line(&mut input).unwrap();
    // println!("Your pick: {}", input);   
    while true {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        println!("Your pick: {}", input);
        if input.contains("1") {
            add();
        }
        else if input.contains("2"){
            subtract();
        }
        else if input.contains("3"){
            multiply();
        }
        else if input.contains("4"){
            divide();
        }
        else{
            return
        }
        println!("\n");
        print_menu();
    }
}

fn print_menu(){
    println!("**************************************************");
    println!("* Option 1: Addition                             *");
    println!("* Option 2: Subtraction                          *");
    println!("* Option 3: Multiplication                       *");
    println!("* Option 4: Division                             *");
    println!("* Option 5: Exit Program                         *");
    println!("**************************************************");
}

fn add()
{
    let mut input = String::new();
    println!("Enter two  numbers to be added\n");
    println!("First: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let num1: i32 = input.trim().parse().expect("Not an integer.");
    println!("\nSecond: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num2: i32 = input.trim().parse().expect("Not an integer");
    println!("\nThe sum is : {}", num1 + num2);
}

fn subtract()
{
    let mut input = String::new();
    println!("Enter two  numbers to be added\n");
    println!("First: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let num1: i32 = input.trim().parse().expect("Not an integer.");
    println!("\nSecond: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num2: i32 = input.trim().parse().expect("Not an integer");
    println!("\nThe sum is : {}", num1 - num2);
}

fn multiply()
{
    let mut input = String::new();
    println!("Enter two  numbers to be added\n");
    println!("First: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let num1: i32 = input.trim().parse().expect("Not an integer.");
    println!("\nSecond: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num2: i32 = input.trim().parse().expect("Not an integer");
    println!("\nThe sum is : {}", num1 * num2);
}

fn divide()
{
    let mut input = String::new();
    println!("Enter two  numbers to be added\n");
    println!("First: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let num1: i32 = input.trim().parse().expect("Not an integer.");
    println!("\nSecond: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num2: i32 = input.trim().parse().expect("Not an integer");
    println!("\nThe sum is : {}", num1 / num2);
}