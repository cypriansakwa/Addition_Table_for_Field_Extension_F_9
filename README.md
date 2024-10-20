## Overview

This program computes the addition table for the finite field extension $\mathbb{F}\_{3^2}$. The field $\mathbb{F}\_{3^2}$ is constructed using the relation $x^2 = 2$ over $\mathbb{F}\_3$. Elements in this field can be represented in the form $a + bx$, where $a, b \in$ {0, 1, 2}, and $x$ is the indeterminate.

## Field Elements in $\mathbb{F}\_{3^2}$

The nine elements in the field $\mathbb{F}_{3^2}$ are:
\[
\{ 0, 1, 2, x, 1 + x, 2 + x, 2x, 1 + 2x, 2 + 2x \}
\]
These elements are stored in the array in the program and used to construct the addition table.

## Program Description

The program defines a struct `F3x2` that represents elements of the field $\mathbb{F}_{3^2}$. Each field element is constructed with coefficients `a` and `b`, where:
 - `a` is the coefficient of the constant term.
 - `b` is the coefficient of $x$.

The program provides a method `add` for adding two field elements according to the arithmetic rules of $\mathbb{F}_{3^2}$.

## Usage

To run the program, ensure you have Rust installed on your machine. (If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it). Then, use the following commands:

>```
>cargo build --release
>cargo run

The program will output the elements of the field and display the addition table.

## Addition Table

The addition table for $\mathbb{F}\_{3^2}$ is computed and printed in the console. The rows and columns of the table represent the elements of the field, and each cell contains the sum of the corresponding elements.

## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.
## Acknowledgments
- Rust

## Clone the repository:

   ```bash
   git clone https://github.com/cypriansakwa/Addition_Table_for_Field_Extension_F_9.git
   cd Addition_Table_for_Field_Extension_F_9
