fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 10; // Safe and idiomatic way to modify the element
    println("First element: {}", v[0]);
}