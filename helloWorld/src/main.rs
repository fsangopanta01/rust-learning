fn main() {
    basics();

    let s = String::from("hello world");
    let y = take_property(s);
    println!("Original chain {}", y);

    let (original, size) = calculate_length(y);
    println!("Original chain {}", original);
    println!("Size {}", size);

    // Refences
    let z = String::from("hello world");
    let size = calculate_length_reference(&z);

    println!("Original chain {}", z);
    println!("Size {}", size);
    // Mutable references
    let mut chain = String::from("hello world");
    modify(&mut chain);
    println!("Mutable chain {}", chain);

}

fn modify(chain: &mut String) {
    chain.push_str("push string")
}

fn take_property(chain: String) -> String {
    println!("take_property chain: {}", chain);
    chain
}

fn calculate_length_reference(chain: &String) -> usize {
    chain.len()
}

fn calculate_length(chain: String) -> (String, usize) {
    let aux = chain.len();
    (chain, aux)
}

fn basics() {
    // By default, variable is not mutable
    let x = 3;
    println!("Value vor non mutable variable: {} ", x);

    // You must indicate if the variable is mutable or not
    let mut y = 5;
    println!("Value of mutable variable: {} ", y);
    y = 23;

    println!("Value of mutable variable changed: {} ", y);

    // Constants

    const PI: f32 = 3.141592653589793;
    println!("Value of constant: {} ", PI);

    // SHADOWING
    let x: char = 'a';
    println!("Initial variable : {} ", x);
    // Same variable with diferente value and type
    let x = 4;
    println!("Reassigned value and type {} ", x);

    //// DATA TYPES

    let result = sum(23, 9);
    println!("Result of type : {}", result);

    let result = is_valid_age(23);
    println!("Result of type : {}", result);

    // TUPLES

    let tupla = (1, 2, 3, -5);
    let (x, y, z, k) = tupla;
    println!("x is {}, y is {}, z is {}", x, y, z);

    let tupla: (char, u32, f64) = ('s', 23, 34.3);

    let k: char = tupla.0;
    println!("k is {}", k);

    // ARRAYS

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("arr element : {}", arr[0]);
    let arr = [3; 5];
    // let arr = [3,3,3,3,3]; --> Equivalent

    // Conditionals
    let number = 10;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    if number < 5 {
        println!("Number is less than 5");
    } else if number == 5 {
        println!("Number is  five");
    } else {
        println!("Number is not less than 5 and five");
    }

    let matriz = [1, 2, 34, 45, 2, 45, 456, 56, 345, 5];

    // Traditional loop
    for numero in matriz.iter() {
        println!("{}", numero);
    }

    // Range for
    for numero in 1..10 {
        println!("{}", numero);
    }

    // Range for inclusive

    for numero in 1..=10 {
        println!("{}", numero);
    }

    // Range for reverse
    println!("Reverse for loop");
    for numero in (1..10).rev() {
        println!("{}", numero);
    }
}
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn is_valid_age(age: i32) -> bool {
    age >= 18
}
