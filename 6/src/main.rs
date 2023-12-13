fn main() {
    // example
    //let times = [7, 15, 30];
    //let distances = [9, 40, 200];

    // part 1
    //let times = [34, 90, 89, 86];
    //let distances = [204, 1713, 1210, 1780];

    // part 2
    let times = [34908986];
    let distances = [204171312101780];

    let product: i64 = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| {
            let min = (0..time).find(|&t| (time - t) * t > distance).unwrap();
            let max = (0..time).rfind(|&t| (time - t) * t > distance).unwrap();
            return max - min + 1;
        })
        .product();

    println!("{product}");
}
