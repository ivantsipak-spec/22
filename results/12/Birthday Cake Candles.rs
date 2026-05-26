use std::io;

fn birthdayCakeCandles(candles: Vec<i32>) -> i32 {
    let max_height = *candles.iter().max().unwrap();

    candles
        .iter()
        .filter(|&&candle| candle == max_height)
        .count() as i32
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();

    let candles: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = birthdayCakeCandles(candles);

    println!("{}", result);
}