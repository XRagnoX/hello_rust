fn shadowing_variable() -> u32 {
    
    let x = "Not shadowed fresh variable"; //if we want to assign again this variable
    println!("variable:\n {x}");

    let x = "Shadowed variable"; //we need to use let to shadow the old variable
    println!("variable:\n {x}");
    
    { //inner scope start
        let x = "If we shadow a variable in the inner scope..."; //shadowing a variable in 
                                                                 //an inner scope
        println!("variable:\n {x}");
    } //inner scope end

    println!("that variable will return to the original value out of the scope"); //will return to
                                                                                  //the original
                                                                                  //value out of
                                                                                  //that scope
    println!("variable:\n {x}");
    
    let error: bool = false; //a bool type can be only true or false
    if(!error) {
    return 0; //old C style fashioned function should always return a variable :P
    }
    return 1;
}

fn main() {
    println!("Hello, world!");

    //Rust is a strong typed language
    
    
    let x: u32 = 1; //Variable declaration.
                    //u32 stands for unsigned integer 32 bit long
                    //Primitive types in rust are Immutable
                    //once declared, variables cannot be updated
    
    println!("The value of the varaible x is: {x}");
    
    //if we want to update the value of a variable, we need to make it Mutable
    //
    let mut x: u32 = 1;
    println!("The value of the Mutable variable x is: {x}");
    x = 2;
    println!("The value of the Mutable variable x is: {x}");

    const Y: u32 = 1; //Constant declaration.
                      //constant variable must always have a data type,
                      //and must always been assigned when declared
    println!("The value of the Constant Y is: {Y}");

    shadowing_variable();
}

