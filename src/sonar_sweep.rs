use crate::prelude::*;

pub async fn eval(input: Vec<String>) -> Result<()> {
    let measurements = parse_input(input);

    println!("count is: {}", count(measurements));

    Ok(())
}


fn parse_input(input: Vec<String>) -> Vec<i32> {
    input.iter()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string().parse::<i32>().expect("always succeeds"))
        .collect()
}

fn count(measurements: Vec<i32>) -> i32 {
    let sums: Vec<i32> = measurements
        .iter()
        .zip(measurements.iter().skip(1))
        .zip(measurements.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect();
    sums.iter().zip(sums.iter().skip(1)).fold(0, |acc, elem| {
        if elem.1 > elem.0 {
            return acc + 1;
        }
        acc
    })
}
