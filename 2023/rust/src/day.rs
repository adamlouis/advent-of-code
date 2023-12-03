pub trait Day {
    fn num(&self) -> u8;
    fn input1(&self) -> String;
    fn input2(&self) -> String;
    fn part1(&self, input: &String) -> String;
    fn part2(&self, input: &String) -> String;
}
