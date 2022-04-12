use aoc_2021::day::day_16::get_day;

const INPUT: &str = r#""#;

#[test]
pub fn part1_1() {{
    assert_eq!(
        get_day()
            .part1("8A004A801A8002F478")
            .expect("Error"),
        "16".to_string()
    );
}}

#[test]
pub fn part1_2() {{
    assert_eq!(
        get_day()
            .part1("620080001611562C8802118E34")
            .expect("Error"),
        "12".to_string()
    );
}}

#[test]
pub fn part1_3() {{
    assert_eq!(
        get_day()
            .part1("C0015000016115A2E0802F182340")
            .expect("Error"),
        "23".to_string()
    );
}}

#[test]
pub fn part1_4() {{
    assert_eq!(
        get_day()
            .part1("A0016C880162017C3686B18A3D4780")
            .expect("Error"),
        "31".to_string()
    );
}}


#[test]
pub fn part2() {{
    assert_eq!(
        get_day()
            .part2(INPUT)
            .expect("Error"),
        "".to_string()
    );
}}
