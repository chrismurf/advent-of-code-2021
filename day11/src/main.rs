use std::fs;
use std::io;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Octopus {
    Unflashed(u8),
    Flashed(),
}

impl Octopus {
    pub fn should_flash(&self) -> bool {
        if let Octopus::Unflashed(count) = self {
            return *count > 9;
        }
        false
    }
}

#[derive(Debug)]
struct OctopusGarden {
    octopi: Vec<Vec<Octopus>>,
    num_octopi: u32,
}

impl OctopusGarden {
    fn get_mut(&mut self, row: i32, col: i32) -> Option<&mut Octopus> {
        if row >= 0
            && row < self.octopi.len() as i32
            && col >= 0
            && col < self.octopi[row as usize].len() as i32
        {
            return Some(&mut self.octopi[row as usize][col as usize]);
        }
        None
    }

    fn increment_neighbors(&mut self, ri: i32, ci: i32) {
        for row in ri - 1..=ri + 1 {
            for col in ci - 1..=ci + 1 {
                if row == ri && col == ci { continue; }
                if let Some(Octopus::Unflashed(count)) = self.get_mut(row, col) {
                    *count += 1;
                }
            }
        }
    }

    fn flash_energetic_octopi(&mut self) -> u32 {
        let mut flash_count: u32 = 0;
        loop {
            let mut anybody_flashed = false;
            for ri in 0..self.octopi.len() {
                for ci in 0..self.octopi[0].len() {
                    if self.octopi[ri][ci].should_flash() {
                        self.octopi[ri][ci] = Octopus::Flashed();
                        anybody_flashed = true;
                        flash_count += 1;
                        self.increment_neighbors(ri as i32, ci as i32);
                    }
                }
            }

            if !anybody_flashed {
                break;
            }
        }
        flash_count
    }

    fn clear_flashed(&mut self) {
        for row in &mut self.octopi {
            for octopus in row {
                match octopus {
                    Octopus::Flashed() => *octopus = Octopus::Unflashed(0),
                    _ => {}
                }
            }
        }
    }

    pub fn step(&mut self) -> u32 {
        // First, the energy level of each octopus increases by 1.
        for row in &mut self.octopi {
            for octopus in row {
                if let Octopus::Unflashed(x) = octopus {
                    *x += 1;
                }
            }
        }
        // Then, any octopus with an energy level greater than 9 flashes.
        let flashed = self.flash_energetic_octopi();
        // Finally, any octopus that flashed has its energy level set to 0.
        self.clear_flashed();
        flashed
    }
}

#[derive(Debug)]
pub struct ParseError;
impl FromStr for OctopusGarden {
    type Err = ParseError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let mut num_octopi = 0;
        let octopi: Vec<Vec<Octopus>> = data
            .split('\n')
            .filter(|l| l.len() != 0)
            .map(|line| {
                line.trim()
                    .bytes()
                    .map(|x| {
                        num_octopi += 1;
                        Octopus::Unflashed(x - ('0' as u8))
                    })
                    .collect::<Vec<Octopus>>()
            })
            .collect();

        Ok(OctopusGarden { octopi, num_octopi })
    }
}

fn main() -> io::Result<()> {
    let input = String::from_utf8(fs::read("input.txt").unwrap()).unwrap();
    let mut garden: OctopusGarden = input.parse().unwrap();

    let mut total_flashed: u32 = 0;
    for _ in 0..100 {
        total_flashed += garden.step();
    }
    println!("Total flashed after 100 steps: {}", total_flashed);

    let mut step_number = 101;
    while garden.step() != garden.num_octopi {
        step_number += 1;
    }
    println!("Took {} steps", step_number);

    Ok(())
}

#[test]
fn test_simple() {
    use indoc::indoc;

    let mut garden = OctopusGarden::from_str(indoc! {"
    11111
    19991
    19191
    19991
    11111
    "})
    .unwrap();

    let step1 = OctopusGarden::from_str(indoc! {"
    34543
    40004
    50005
    40004
    34543
    "})
    .unwrap();

    let step2 = OctopusGarden::from_str(indoc! {"
    45654
    51115
    61116
    51115
    45654
    "})
    .unwrap();

    garden.step();
    assert_eq!(garden.octopi, step1.octopi);
    garden.step();
    assert_eq!(garden.octopi, step2.octopi);
}

#[test]
fn test_examples() {
    use indoc::indoc;

    let mut garden = OctopusGarden::from_str(indoc! {"
    5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526
    "})
    .unwrap();

    let step1 = OctopusGarden::from_str(indoc! {"
    6594254334
    3856965822
    6375667284
    7252447257
    7468496589
    5278635756
    3287952832
    7993992245
    5957959665
    6394862637
    "})
    .unwrap();

    garden.step();
    assert_eq!(garden.octopi, step1.octopi);
}
