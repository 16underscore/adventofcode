use std::fs;

struct Cuboid {
	l: u32,
	w: u32,
	h: u32,
}

impl Cuboid {
	fn new(l: u32, w: u32, h: u32) -> Cuboid {
		Cuboid { l, w, h }
	}

	fn surface(&self) -> u32 {
		2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
	}

	fn volume(&self) -> u32 {
		self.l * self.w * self.h
	}

	fn to_array(&self) -> [u32; 3] {
		[self.l, self.w, self.h]
	}
}

fn main() {
	let input = fs::read_to_string("2015/two").unwrap();
	let boxes: Vec<Cuboid> = input
		.split('\n')
		.map(|row| {
			let dimension: Vec<u32> = row.split('x').map(|edge| edge.parse().unwrap()).collect();
			Cuboid::new(
				*dimension.get(0).unwrap(),
				*dimension.get(1).unwrap(),
				*dimension.get(2).unwrap(),
			)
		})
		.collect();
	part1(&boxes);
	part2(&boxes);
}

fn part1(boxes: &Vec<Cuboid>) {
	let mut sum = 0;
	for c in boxes {
		let mut cuboid = c.to_array();
		cuboid.sort();
		sum += c.surface() + cuboid.get(0).unwrap() * cuboid.get(1).unwrap();
	}
	println!("The result for the first part is: {}", sum);
}

fn part2(boxes: &Vec<Cuboid>) {
	let mut sum = 0;
	for c in boxes {
		let mut cuboid = c.to_array();
		cuboid.sort();
		sum += 2 * cuboid.get(0).unwrap() + 2 * cuboid.get(1).unwrap() + c.volume();
	}
	println!("The result for the second part is: {}", sum);
}
