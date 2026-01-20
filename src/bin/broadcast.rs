use ndarray::prelude::*;

fn main() {
    let a = 10;
    let b = 10;

    let x = Array1::linspace(0.0, 500.0, a);
    let y = Array1::linspace(0.0, 500.0, b);

    println!("{:?}", x);
    println!("{:?}", y);

    let x_col = x.view().insert_axis(Axis(1));
    let y_col = y.view().insert_axis(Axis(0));
    let z = &x_col * &x_col + &y_col * &y_col;

    println!("{:#?}", z);
}
