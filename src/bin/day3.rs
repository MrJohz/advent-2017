#![feature(conservative_impl_trait)]
#![feature(iterator_step_by)]

const LOCATION: usize = 15; //include!("inputs/day3.txt");

#[derive(Debug)]
struct Cartesian {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Rotatory {
    circle: usize,
    side: usize,
    height: usize,
}

impl Rotatory {
    fn new_from_int(location: usize) -> Self {
        let (min, max, depth)  = {
            let mut min = 0;
            let mut max = 0;
            let mut depth = 0;
            for (idx, (l_bound, u_bound)) in squares().enumerate() {
                if location > l_bound && location <= u_bound {
                    min = l_bound;
                    max = u_bound;
                    depth = idx;
                    break;
                }
            };
            (min, max, depth)
        };

        let side_location = location - min;
        let side_length = (max - min) / 4;

        let (side, height) = if side_location > 0 && side_location < side_length {
            (0, side_location)
        } else if side_location >= side_length && side_location < side_length * 2 {
            (1, side_location - side_length)
        } else if side_location >= side_length * 2 && side_location < side_length * 3 {
            (2, side_location - side_length * 2)
        } else if side_location >= side_length * 3 && side_location < side_length * 4 {
            (3, side_location - side_length * 3)
        } else {
            (0, 0)
        };

        Self::new_from_coordinates(depth, side, height)
    }

    fn new_from_coordinates(circle: usize, side: usize, height: usize) -> Self {
        Self {
            circle, side, height
        }
    }

    fn distance_to_centre(&self) -> usize {
        self.circle +
            if self.height == self.circle {
                0
            } else if self.height > self.circle {
                self.height - self.circle
            } else {
                self.circle - self.height
            }
    }

    fn as_cartesian(&self) -> Cartesian {
        let fixed = self.circle as isize;
        let variable = self.height as isize - self.circle as isize;

        match self.side {
            0 => Cartesian { x: fixed, y: variable },
            2 => Cartesian { x: -fixed, y: -variable },
            1 => Cartesian { x: -variable, y: fixed },
            3 => Cartesian { x: variable, y: -fixed },
            _ => panic!("self.side out of bounds")
        }
    }

    fn as_int(&self) -> usize {
        let (min, max) = squares().nth(self.circle).unwrap();
        if self.height == self.side && self.height == 0 {
            max
        } else {
            min + (self.circle * self.side * 2) + self.height
        }
    }
}

fn squares() -> impl Iterator<Item=(usize, usize)> {
    (1..)
        .step_by(2)
        .map(|num| {
            if num == 1 {
                (0, 1)
            } else {
                ((num - 2) * (num - 2), num * num)
            }
        })
}

fn main() {
    let coords = Rotatory::new_from_int(LOCATION);
    println!("LOCATION: {:?} (coord: {:?}) DISTANCE: {:?}", LOCATION, coords, coords.distance_to_centre());

    println!("{:?}", coords.as_int());

    for i in 1..20 {
        let rotary = Rotatory::new_from_int(i);
        println!("{:?} -> {:?} -> {:?}", i, rotary, rotary.as_cartesian());
    }
}
