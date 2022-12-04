pub fn part1(input: &str) -> u32 {
    let mut occurences = vec![];
    input.lines().filter(|x| !x.is_empty()).for_each(|number| {
        number.chars().enumerate().for_each(|(i, digit)| {
            while occurences.len() <= i {
                occurences.push(0);
            }
            if digit == '1' {
                occurences[i] += 1;
            } else {
                occurences[i] -= 1;
            }
        });
    });
    let epsilon = u32::from_str_radix(
        &occurences
            .iter()
            .map(|x| if *x >= 0 { "1" } else { "0" })
            .collect::<Vec<_>>()
            .join(""),
        2,
    )
    .unwrap();
    let gamma = u32::from_str_radix(
        &occurences
            .iter()
            .map(|x| if *x >= 0 { "0" } else { "1" })
            .collect::<Vec<_>>()
            .join(""),
        2,
    )
    .unwrap();
    epsilon * gamma
}

fn get_digit(input: &[&str], index: usize) -> Vec<char> {
    input
        .iter()
        .map(|item| item.chars().nth(index).unwrap())
        .collect()
}

fn get_most_common(input: &[char]) -> char {
    if input.iter().filter(|x| **x == '0').count() > input.len() / 2 {
        '0'
    } else {
        '1'
    }
}

pub fn part2(input: &str) -> u32 {
    let mut ogr_data: Vec<&str> = input.lines().collect();
    let mut co2_data: Vec<&str> = input.lines().collect();
    for i in 0..input.lines().next().unwrap().len() {
        let most_common_ogr = get_most_common(&get_digit(&ogr_data, i));
        if ogr_data.len() > 1 {
            ogr_data.retain(|x| x.chars().nth(i).unwrap() == most_common_ogr);
        }
        let most_common_co2 = get_most_common(&get_digit(&co2_data, i));
        if co2_data.len() > 1 {
            co2_data.retain(|x| x.chars().nth(i).unwrap() != most_common_co2);
        }
    }
    let ogr = u32::from_str_radix(ogr_data.first().unwrap(), 2).unwrap();
    let co2 = u32::from_str_radix(co2_data.first().unwrap(), 2).unwrap();
    ogr * co2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n"
        ), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n"
        ), 230);
    }
}
