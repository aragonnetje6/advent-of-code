pub fn part1(input: &str) -> u32 {
    let mut occurences = vec![];
    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .for_each(|number| {
            number.chars().enumerate().for_each(|(i, digit)| {
                while occurences.len() <= i {
                    occurences.push(0);
                }
                if digit == '1' {
                    occurences[i] += 1;
                } else {
                    occurences[i] -= 1;
                }
            })
        });
    println!("{occurences:?}");
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
