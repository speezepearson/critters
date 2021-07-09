use rand::distributions::Distribution;

struct World {
  age_even: bool,
  cells: [[bool; 32]; 32],
}

impl World {

  fn step(&mut self) {
    let offset = if self.age_even {0} else {1};
    let tl_corners = (offset..offset+32).step_by(2).flat_map(|row| (offset..offset+32).step_by(2).map(move |col| (row, col))).collect::<Vec<(usize, usize)>>();
    for (row, col) in tl_corners {
      let nrow = (row+1) % 32;
      let ncol = (col+1) % 32;
      let (tl, tr, bl, br) = (self.cells[row][col], self.cells[row][ncol], self.cells[nrow][col], self.cells[nrow][ncol]);
      let n_alive = (tl as i8)+(tr as i8)+(bl as i8)+(br as i8);
      if n_alive == 3 {
        self.cells[row] [col]  = !br;
        self.cells[row] [ncol] = !bl;
        self.cells[nrow][col]  = !tr;
        self.cells[nrow][ncol] = !tl;
      } else if n_alive != 2 {
        self.cells[row] [col]  = !tl;
        self.cells[row] [ncol] = !tr;
        self.cells[nrow][col]  = !bl;
        self.cells[nrow][ncol] = !br;
      }
    }
    self.age_even = !self.age_even;
  }
}

fn render_grid(world: &World) -> String {
  world.cells.into_iter()
    .map(|row| row.into_iter().map(|cell| if *cell ^ world.age_even {"#"} else {" "}).collect::<Vec<&str>>().join(""))
    .collect::<Vec<String>>()
    .join("\n")
}

fn main() {
  let mut world = World {
    age_even: false,
    cells: [[false; 32]; 32],
  };

  let mut rng = rand::thread_rng();
  let rowcol_dist = rand::distributions::Uniform::new_inclusive(16-5, 16+4);
  for _ in 0..50 {
    let row = rowcol_dist.sample(&mut rng);
    let col = rowcol_dist.sample(&mut rng);
    world.cells[row][col] = true;
  }

  loop {
    println!("\n\n-------------------------------------------------\n\n{}", render_grid(&world));
    world.step();
    std::thread::sleep(std::time::Duration::from_millis(50));
  }
}
