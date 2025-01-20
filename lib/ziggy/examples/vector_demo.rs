use ziggy::Vector3D;

fn main() {
    let v1 = Vector3D::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::new(4.0, 5.0, 6.0);

    println!("Vector 1: {}", v1);
    println!("Vector 2: {}", v2);
    println!("Dot product: {}", v1.dot(&v2));
    println!("Magnitude of v1: {}", v1.magnitude());
}
