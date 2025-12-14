advent_of_code::solution!(2);

fn is_repeated_twice(n: u64) -> bool {
    let num = n.to_string();
    let first = &num[0..num.len() / 2];
    let second = &num[num.len() / 2..];
    if first == second {
        return true;
    }
    false
}

fn slice_into_n_pieces(num: &String, split: usize) -> Result<Vec<&str>, bool> {
    let mut vec: Vec<&str> = Vec::with_capacity(split);
    if num.len() % split != 0 {
        return Err(false);
    }
    let split_len = num.len() / split;
    for i in 0..split {
        vec.push(&num[split_len * i..(split_len * i + split_len)]);
    }
    Ok(vec)
}

fn is_repeated_n_times(n: u64) -> bool {
    let num = n.to_string();
    for i in 2..num.len() + 1 {
        let split = usize::try_from(i).expect("couldn't convert to usize");
        let parts = match slice_into_n_pieces(&num, split) {
            Ok(parts) => parts,
            Err(_) => continue,
        };
        let first_part = parts[0];
        let mut are_all_parts_equal = true;
        for part in parts {
            if part != first_part {
                are_all_parts_equal = false;
                break;
            }
        }
        if are_all_parts_equal {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.split("\n").collect::<Vec<_>>()[0].split(",");
    let mut sum = 0;
    for r in ranges {
        let range: Vec<&str> = r.split("-").collect();
        let start_num = range[0].parse::<u64>().unwrap();
        let end_num = range[1].parse::<u64>().unwrap();
        for n in start_num..end_num + 1 {
            if is_repeated_twice(n) {
                sum += n;
                println!("Repeated: {n}");
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input.split("\n").collect::<Vec<_>>()[0].split(",");
    let mut sum = 0;
    for r in ranges {
        let range: Vec<&str> = r.split("-").collect();
        let start_num = range[0].parse::<u64>().unwrap();
        let end_num = range[1].parse::<u64>().unwrap();
        for n in start_num..end_num + 1 {
            if is_repeated_n_times(n) {
                sum += n;
                println!("Repeated: {n}");
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
