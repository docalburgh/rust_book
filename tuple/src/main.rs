//a tuple is a way of grouping together values with different types into one compound type.
//tuples have a fixed length. Once declared they cannot grow or shrink in size.
//this program first binds the variable tup to a tuple. Then, through the let keyword,
//the variable tup gets deconstructed into three different variables: x, y, and z.

fn main() {
    let tup = (500, 6.4, 1); //32 bit signed integer, 64 bit floating point number, and unsigned 8 bit integer, respectively
    
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}

