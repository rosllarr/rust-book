use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop can't used here, because v have moved in to closure
    // this might be drop reference to invalid variable
    //drop(v);

    handle.join().unwrap();
}
