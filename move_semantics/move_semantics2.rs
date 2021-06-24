// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
    // // Solution 1: Deep copy with .clone()
    let vec0 = Vec::new();
    let mut vec1 = fill_vec(vec0.clone()); // Solution 1: make deap copy of heap data
    
    // // Solution 2: borrow and copy
    // let vec0 = Vec::new();
    // let mut vec1 = fill_vec_borrow(&vec0); 
    
    // // Solution 3: Mutable borrow
    // let mut vec0 = Vec::new();
    // fill_vec_mut_borrow(&mut vec0); 


    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

// Solution 2
fn fill_vec_borrow(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

// Solution 3
fn fill_vec_mut_borrow(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
