use std::io;

pub fn read_string_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn read_f32_input(prompt: &str) -> f32 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string().parse::<f32>().unwrap()
}

pub fn bubble_sort<T, F>(mut vec: Vec<T>, mut compare: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T, &T) -> bool,
{
    loop {
        let mut swapped = false;

        let mut i = 0;
        while i < vec.len() {
            if i >= vec.len() - 1 { break; }
            if compare(&vec[i], &vec[i + 1]) {
                swapped = true;
                let value = vec[i].clone();
                vec[i] = vec[i + 1].clone();
                vec[i + 1] = value;
                break;
            }
            i += 1;
        }

        if !swapped { break; }
    }

    vec
}
