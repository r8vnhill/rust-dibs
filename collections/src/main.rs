mod lazy;

use lazy::EvenNumbers;

fn main() {
    let mut even_numbers = EvenNumbers::new();
    let even_numbers: Vec<u32> = EvenNumbers::new().take(5).collect();
    println!("{:?}", even_numbers);
    /* Output:
     [0, 2, 4, 6, 8]
     */
}
