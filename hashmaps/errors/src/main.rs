use std::fs::File;
use std::io;
use std::io::{ErrorKind,Read};

fn main() {
    println!("errrorrrrrs!");

    shortened_propogating_errors();
    propogating_errors();
    result();
    errors();
    out_of_bounds_panic();
    panics();
}

fn shortned_propogating_errors() -> Result<String, io::Error> {
    // '?' will propogate the error if an error occurs, otherwise will
    // return the Ok(result).
    // There is a difference between the manual expanded version of '?';
    // error values used with '?' go through the 'From' trait (definied
    // in the stdlib). When '?' calls the 'from' function, the error type
    // is received is converted into the error type defined in the return
    // type of the current function.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s);
}

fn propogating_errors() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(fd) => fd,
        // Propogate the error to caller.
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn result() {
    //let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn errors() {
    /*
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    */
    let filename = "not-existent-file.txt";
    let f = File::open(filename);
    let f = match f {
        Ok(file) => file,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            match File::create(filename) {
                Ok(fd) => fd,
                Err(e) => {
                    panic!("unable to create new file: {:?}", e);
                },
            }
        },
        Err(err) => {
            panic!("unable to open file: {:?}", err);
        },
    };
}

fn out_of_bounds_panic() {
    let v = vec![1, 2, 3];
    v[99];
}

fn panics() {
    panic!("crash and burn!");
}
