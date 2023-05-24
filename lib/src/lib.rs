pub use paste::paste;

#[macro_export]
macro_rules! solution {
    ( $day:literal, 1 ) => {{
        use lib::paste;
        paste! {
            use [<p $day>] as p;
            println!(
                "Day {} part 1: {}",
                $day,
                p::part1(include_str!(concat!("../input/p", stringify!($day))))
            )
        }
    }};
    ( $day:literal, 2 ) => {{
        use lib::paste;
        paste! {
            use [<p $day>] as p;
            println!(
                "Day {} part 2: {}",
                $day,
                p::part2(include_str!(concat!("../input/p", stringify!($day))))
            )
        }
    }};
}
