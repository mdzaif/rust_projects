pub fn run()
{
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    
    // With non-primitives, if  you assign another variable  to piece of data, the first
    //variable will no longer hold that vaue. You will need to use a reference (&) to point to
    //the resource
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Vectors: {:?}", (&vec1, vec2));
}
