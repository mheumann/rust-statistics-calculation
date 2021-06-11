use std::collections::HashMap;
use std::io;

fn main() {
    let mut numbers: Vec<usize> = vec![];

    println!("Calculate average, median and mode of a series of numbers");
    println!("Insert a number or enter 'q', if you don't want to add another number.");
    loop{
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("nothing inserted");

        if input.trim() == "q" {
            break;
        } else {
            numbers.push(match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            });
        }
    }

    if numbers.len() == 0 {
        println!("You didn't insert any numbers. Restart the program!");
    } else {
        numbers.sort_unstable();

        let avg = calc_avg(&numbers);
        println!("The average is: {}", avg);

        let median = calc_median(&numbers);
        println!("But the median is: {}", median);

        let mode = calc_mode(&numbers);
        println!("And the mode is: {}", mode);
    }
}

fn calc_avg(numbers: &Vec<usize>) -> usize {
    let mut sum = 0;
    let num_size = numbers.len();

    for number in numbers {
        sum += number;
    }
    return sum / num_size;
}

fn calc_median(numbers: &Vec<usize>) -> usize {
    let num_size = numbers.len();
    let middle = num_size / 2;

    return if num_size % 2 == 1 {
        numbers[middle]
    } else {
        (numbers[middle - 1] + numbers[middle]) / 2
    }
}

fn calc_mode(numbers: &Vec<usize>) -> usize {
    let mut map: HashMap<usize, u32> = HashMap::new();

    for number in numbers {
        let count = map.entry(*number).or_insert(0);
        *count += 1;
    }

    let mut highest_val= 0;
    let mut highest_count = 0;
    for (num, count) in map.iter() {
        if *count > highest_count {
            highest_count = *count;
            highest_val = *num;
        }
    }

    return highest_val;
}