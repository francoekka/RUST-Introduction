use std::io;
pub fn run(){
    let mut choice:i32 = 1;
    while choice != 0{

        println!("\n*How to :
        1.Print To Console
        2.Print Positional and Named Arguments
        3.Print Variable
        4.Print Arithmetic Operations
        0.Exit");
        let mut input_string = String::new();
        io::stdin()
        .read_line(&mut input_string)
        .expect("cannot read from stdin");
        choice = input_string.trim().parse().expect("Not a valid number");

            match choice{

                //print
                1 =>{println!("\n-> Printing a Statement\n\n
                    Code :
                    println!(\"hello its rust !\");\n");
                    println!("Output:\n hello its rust !");}

                //print positional argument
                2 =>{println!("
                    Code :
                    println!(\"{{}} is {{}} years old\",\"Franco\",25); // Basic formatting
                    println!(\"{{1}} is {{0}} years old\",\"Franco\",25); // Positional Arguments
                    println!(\"{{name}} is {{age}} years old\",name=\"Franco\",age=25);// Named Arguments\n");
                    
                    println!("Output:\n{} is {} years old","Franco",25);
                    println!("{1} is {0} years old","Franco",25);
                    println!("{name} is {age} years old",name="Franco",age=25);}

                //print variable
                3 =>{println!("\n-> Printing Variables\n");
                    println!("
                    Code :
                    let mut n = 2; //mut used to make mutable
                    println!(\"n = {{}}\",n);
                    n = 3;
                    println!(\"n = {{}}\",n);
                    let b = n;
                    println!(\"b = {{}}\",b);\n");
                    println!("Output:");
                    let mut n = 2; //mut used to make mutable
                    println!("n = {}",n);
                    n = 3;
                    println!("n = {}",n);
                    let b = n;
                    println!("b = {}\n",b);}

                //print arithmetic
                4 =>{println!("-> Printing Arithmetic Operation");
                    println!("
                    Code :
                    let  n = 2;
                    let  b = 3;
                    println!(\"{{}} + {{}} = {{}}\",n,b,n+b);
                    println!(\"{{}} - {{}} = {{}}\",n,b,n-b);
                    println!(\"{{}} * {{}} = {{}}\",n,b,n*b);
                    println!(\"{{}} / {{}} = {{}}\",n,b,n/b);\n");
            
                    let  n = 2;
                    let  b = 3;
                    println!("Output:");
                    println!("{} + {} = {}",n,b,n+b);
                    println!("{} - {} = {}",n,b,n-b);
                    println!("{} * {} = {}",n,b,n*b);
                    println!("{} / {} = {}",n,b,n/b);}
                0 =>{println!("\n");}
                _ => println!("Wrong Option !!..") }

    }
}