// This bug occurs because const means that the value is copied to every instance
// at compile time. 
//
// What I wanted to do is to create a "constantly increasing value", i.e. 
// a value that is private to this module and gets increased by a function.
//
// This is very useful if you generate unique keys for a given value, for example.

const VALUE: AtomicUsize = AtomicUsize::new(0);

fn generate_unique_id() -> usize {
    // note: VALUE gets **copied** here if declared as const
    VALUE.fetch_add(1, Ordering::SeqCst)
}

fn main() {

    // expected: 0, 1, 2
    // actual: 0, 0, 0
    
    println!("{}", generate_unique_id());
    println!("{}", generate_unique_id());
    println!("{}", generate_unique_id());
    
}

// What fixes this is switching `const VALUE` to `static VALUE`,
// since static is not copied to every invocation location
//
// A possible fix would be to make AtomicUsize not Copy-able, 
// but that would probably break a lot of code
