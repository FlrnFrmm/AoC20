use anyhow::Result;
use std::io::Read;

fn main() -> Result<()> {
    let problem_cases = parse_input()?;

    let count_part_one = problem_cases.iter().filter(|p| p.correct_for_first_policy()).count();
    let count_part_two = problem_cases.iter().filter(|p| p.correct_for_second_policy()).count();

    println!("{}", count_part_one);
    println!("{}", count_part_two);

    Ok(())
}

fn parse_input() -> Result<Vec<ProblemCase>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    input.lines()
        .map(|problem_case| ProblemCase::new_from_input(problem_case))
        .collect()
}

struct ProblemCase {
    min: usize,
    max: usize,
    target: char,
    password: String
}

impl ProblemCase {
    fn new_from_input(problem_case: &str) -> Result<Self> {
        let case_parts: Vec<&str> = problem_case.split(' ').collect();

        let min = case_parts[0].split('-').collect::<Vec<&str>>()[0].parse::<usize>()?;
        let max = case_parts[0].split('-').collect::<Vec<&str>>()[1].parse::<usize>()?;
        let target = case_parts[1].chars().next().unwrap();

        Ok(ProblemCase { min, max, target, password: String::from(case_parts[2]) })
    }

    fn correct_for_first_policy(&self) -> bool {
        let target_count = self.password.chars()
            .filter(|x| *x == self.target).count();

        target_count >= self.min && target_count <= self.max 
    }

    fn correct_for_second_policy(&self) -> bool {
        let first_pos = self.password.chars()
            .skip(self.min - 1)
            .next()
            .map_or(false, |c| c == self.target);
        let second_pos = self.password.chars()
            .skip(self.max - 1)
            .next()
            .map_or(false, |c| c == self.target);
        !(first_pos && second_pos) && (first_pos || second_pos)
    }
}