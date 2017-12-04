enum State {
  GoRight,
  GoUp,
  GoLeft,
  GoDown
}

// debugging purpose
fn display_spiral(spiral: &Vec<Vec<i32>>) {
  let len = spiral[0].len();

  for row in 0..len {
    println!("");

    for column in 0..len {
      print!(" {} ", spiral[row][column]);
    }
  }
}

// Part 1 functions

fn compute_next_pos(spiral: &Vec<Vec<i32>>, (i, j): (usize, usize)) -> (i32, (usize, usize)) {
  let max_index = spiral[0].len() - 1;
  let mut min = (spiral[i][j], (i, j));
  let mut x;
  let mut y;

  if i > 0 {
    x = j;
    y = i - 1;

    if spiral[y][x] > 0 && min.0 > spiral[y][x] {
      min = (spiral[y][x], (y, x));
    }
  }

  if i < max_index {
    x = j;
    y = i + 1;

    if spiral[y][x] > 0 && min.0 > spiral[y][x] {
      min = (spiral[y][x], (y, x));
    }
  }

  if j > 0 {
    x = j - 1;
    y = i;

    if spiral[y][x] > 0 && min.0 > spiral[y][x] {
      min = (spiral[y][x], (y, x));
    }
  }

  if j < max_index {
    x = j + 1;
    y = i;

    if spiral[y][x] > 0 && min.0 > spiral[y][x] {
      min = (spiral[y][x], (y, x));
    }
  }

  return min;
}

fn compute_part_1(spiral: &Vec<Vec<i32>>, (i, j): (usize, usize)) -> i32 {
  let mut prev_value = 0;
  let mut current = (spiral[i][j], (i, j));
  let mut count = 0;

  while current.0 != 1 && current.0 != prev_value {
    prev_value = current.0;
    current = compute_next_pos(&spiral, current.1);
    count += 1;

    // println!("{} - {} {}", current.0, (current.1).0, (current.1).1);
  }

  return count;
}

fn build_spiral_part_1(input: i32) -> (Vec<Vec<i32>>, (usize, usize)) {
  use State::*;

  let side_length: usize = (input as f32).sqrt().ceil() as usize + 1;
  let mut spiral = vec![vec![0 as i32; side_length]; side_length];
  let start_index: usize = (((side_length / 2) as f32).floor()) as usize;

  // fills array
  let mut x = start_index;
  let mut y = start_index;
  let mut state = GoRight;

  for n in 1..(input + 1) {
    // write current value
    spiral[y][x] = n;

    // save the last position values
    if n == input {
      break;
    }

    // manage states
    match state {
      GoRight => {
        x += 1;

        if spiral[y - 1][x] == 0 {
          state = GoUp;
        }
      },
      GoUp => {
        y -= 1;

        if spiral[y][x - 1] == 0 {
          state = GoLeft;
        }
      },
      GoLeft => {
        x -= 1;

        if spiral[y + 1][x] == 0 {
          state = GoDown;
        }
      },
      GoDown => {
        y += 1;

        if spiral[y][x + 1] == 0 {
          state = GoRight;
        }
      }
    }
  }

  return (spiral, (y, x));
}


// Part 2 functions

fn compute_next_value(spiral: &Vec<Vec<i32>>, (i, j): (usize, usize)) -> i32 {
  let max_index: usize = spiral[0].len() - 1;

  // compute bounds
  let min_x: usize = if j > 0 { j - 1 } else { j };
  let min_y: usize = if i > 0 { i - 1 } else { i };
  let max_x: usize = if j < max_index { j + 1 } else { j };
  let max_y: usize = if i < max_index { i + 1 } else { i };

  // sum all adjacent boxes
  let mut sum = 0;
  for y in min_y..max_y+1 {
    for x in min_x..max_x+1 {
      if x == j && y == i {
        continue;
      }

      // println!("{} - {} {}", spiral[y][x], x, y);
      // println!("{}", sum);

      sum += spiral[y][x];
    }
  }

  return sum;
}

fn compute_part_2(input: i32) -> i32 {
  use State::*;

  let side_length: usize = (input as f32).sqrt().ceil() as usize + 1;
  let mut spiral = vec![vec![0 as i32; side_length]; side_length];
  let start_index: usize = (((side_length / 2) as f32).floor()) as usize;

  // fills array
  let mut x = start_index;
  let mut y = start_index;
  let mut state = GoRight;
  let mut current: i32 = input;

  for n in 1..(input + 1) {
    // compute next value
    if n <= 2 {
      current = 1;
    } else {
      current = compute_next_value(&spiral, (y, x));
    }

    // println!("{}", current);

    // save the last position values
    if current > input {
      return current;
    }

    // write current value
    spiral[y][x] = current;

    // manage states
    match state {
      GoRight => {
        x += 1;

        if spiral[y - 1][x] == 0 {
          state = GoUp;
        }
      },
      GoUp => {
        y -= 1;

        if spiral[y][x - 1] == 0 {
          state = GoLeft;
        }
      },
      GoLeft => {
        x -= 1;

        if spiral[y + 1][x] == 0 {
          state = GoDown;
        }
      },
      GoDown => {
        y += 1;

        if spiral[y][x + 1] == 0 {
          state = GoRight;
        }
      }
    }
  }

  // should not append
  return current;
}

fn main() {
  let input: i32 = 347991;
  // let input: i32 = 1024;

  // Part 1
  let (spiral, coord) = build_spiral_part_1(input);
  let output_part_1 = compute_part_1(&spiral, coord);
  println!("output part 1 : {}", output_part_1);

  // Part 2
  let output_part_2 = compute_part_2(input);
  println!("output part 2 : {}", output_part_2);
}