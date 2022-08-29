use std::io;

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?; // ? here return Err imidiately if read_line gives error
    Ok(buffer.trim().to_owned()) // to_owned since we are returning  String as success
}

fn main() {
    let mut all_inputs = vec![];
    let mut times_input = 0;
    while times_input < 2 {
        match get_input() {
            Ok(input_string) => {
                all_inputs.push(input_string);
                times_input += 1;
            }
            Err(e) => println!("error: {:?}", e),
        }
    }

    for output in all_inputs{
        println!("original: {:?}, upperCased: {:?}", output, output.to_uppercase());
    }
}
