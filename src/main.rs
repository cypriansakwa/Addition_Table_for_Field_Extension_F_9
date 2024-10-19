use std::fmt;

/// A struct to represent elements of the field \( \mathbb{F}_{3^2} \)
#[derive(Clone, Copy, Debug)]
struct F3x2 {
    a: u8, // coefficient for 1
    b: u8, // coefficient for x
}

impl fmt::Display for F3x2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.a, self.b) {
            (0, 0) => write!(f, "0"),  // when both coefficients are zero, print 0
            (a, 0) => write!(f, "{}", a),  // when b=0, print just the constant part
            (0, 1) => write!(f, "x"),  // when a=0 and b=1, print just "x"
            (0, b) => write!(f, "{}x", b),  // when a=0 and b != 1, print "bx"
            (1, 1) => write!(f, "1 + x"),  // when a=1 and b=1, print "1 + x"
            (a, 1) => write!(f, "{} + x", a),  // when b=1, print constant + "x"
            (a, b) => write!(f, "{} + {}x", a, b),  // otherwise, print "a + bx"
        }
    }
}

impl F3x2 {
    /// Creates a new field element
    fn new(a: u8, b: u8) -> Self {
        F3x2 { a: a % 3, b: b % 3 }
    }

    /// Adds two elements of the field \( \mathbb{F}_{3^2} \)
    fn add(self, other: F3x2) -> F3x2 {
        F3x2 {
            a: (self.a + other.a) % 3,
            b: (self.b + other.b) % 3,
        }
    }
}

fn main() {
    // Define the 9 elements of \( \mathbb{F}_{3^2} \)
    let field_elements = [
        F3x2::new(0, 0), // 0
        F3x2::new(1, 0), // 1
        F3x2::new(2, 0), // 2
        F3x2::new(0, 1), // x
        F3x2::new(1, 1), // x + 1
        F3x2::new(2, 1), // x + 2
        F3x2::new(0, 2), // 2x
        F3x2::new(1, 2), // 2x + 1
        F3x2::new(2, 2), // 2x + 2
    ];

    // Print the field elements
    println!("Field Elements in \\mathbb{{F}}_{{3^2}}:");
    for (i, elem) in field_elements.iter().enumerate() {
        println!("Element {}: {}", i, elem);
    }

    // Draw the addition table
    println!("\nAddition Table for \\mathbb{{F}}_{{3^2}}:\n");

    // Print the header
    print!("     ");
    for elem in field_elements.iter() {
        print!("{:>10} ", elem);
    }
    println!();

    // Print each row of the addition table
    for elem1 in field_elements.iter() {
        print!("{:>3} |", elem1); // print row label
        for elem2 in field_elements.iter() {
            let sum = elem1.add(*elem2);
            print!("{:>10} ", sum);
        }
        println!();
    }
}
