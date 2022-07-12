
use std::ops;

#[derive(Clone)]
struct Vec3 {
  position: [f32;3],
  rgb: [i32;3],
}

enum Vec3Init<'a> {
  #[allow(dead_code)]
  Vec3Parameter(&'a Vec3),
  PositionParameter(f32, f32, f32),
}

#[allow(dead_code)]
impl Vec3 {
  fn new(vec3_init: Vec3Init) -> Vec3 {
    
    match vec3_init {
      Vec3Init::Vec3Parameter(vec3) => {
        return Vec3 {
          position: vec3.position,
          rgb: vec3.rgb,
        }
      },
      Vec3Init::PositionParameter(a, b, c) => {
        return Vec3 {
          position: [a, b, c],
          rgb: [a.round() as i32, b.round() as i32, c.round() as i32],
        }
      },
    }
  }
  fn x(&self) -> f32 {
    return self.position[0];
  }
  fn y(&self) -> f32 {
    return self.position[1];
  }
  fn z(&self) -> f32 {
    return self.position[2];
  }
  fn r(&self) -> i32 {
    return self.rgb[0];
  }
  fn g(&self) -> i32 {
    return self.rgb[1];
  }
  fn b(&self) -> i32 {
    return self.rgb[2];
  }
  fn length(&self) -> f32 {
    return (self.position[0].powi(2) + self.position[1].powi(2) + self.position[2].powi(2)).sqrt();
  }
  fn make_unit(&mut self) {
    let k = 1.0 / self.length();
    (*self).position[0]/=k;
    (*self).position[1]/=k;
    (*self).position[2]/=k;
  }
}

trait Vec3Operations {
  fn cross(&self, rhs: &Vec3) -> Vec3;
  fn dot(&self, rhs: &Vec3) -> Vec3;
  // fn unit(&self) -> &Vec3) -> Vec3;
}

impl Vec3Operations for Vec3 {
  fn cross(&self, rhs: &Vec3) -> Vec3 {
    return Vec3 {
      position: [
        self.position[1] * rhs.position[2] - self.position[2] * rhs.position[1], 
        -(self.position[0] * rhs.position[2] - self.position[2] * rhs.position[0]), 
        self.position[0] * rhs.position[1] - self.position[1] * rhs.position[0]
        ],
      rgb:  [(self.rgb[0] + rhs.rgb[0]) / 2, (self.rgb[1] + rhs.rgb[1]) / 2, (self.rgb[2] + rhs.rgb[2]) / 2],
    }
  }

  fn dot(&self, rhs: &Vec3) -> Vec3 {
    return Vec3 {
      position: [
        self.position[0] * rhs.position[0],
        self.position[1] * rhs.position[1],
        self.position[2] * rhs.position[2]
      ],
      rgb: [
        (self.rgb[0] + rhs.rgb[0]) / 2,
        (self.rgb[1] + rhs.rgb[1]) / 2,
        (self.rgb[2] + rhs.rgb[2]) / 2,
      ]
    }
  }
}

trait DebugTooling {
  fn debug(&self);
}

impl DebugTooling for Vec3 {
  fn debug(&self) {
    println!("Position Array is {:?}", self.position);
    println!("RGB Array is {:?}", self.rgb);
  }
}

impl ops::Add<&Vec3> for &Vec3 {
  type Output = Vec3;
  fn add(self, rhs: &Vec3) -> Vec3 {
    Vec3 {
      position: [self.position[0] + rhs.position[0], self.position[1] + rhs.position[1], self.position[2] + rhs.position[2]],
      rgb:  [self.rgb[0] + rhs.rgb[0], self.rgb[1] + rhs.rgb[1], self.rgb[2] + rhs.rgb[2]],
    }
  }
}

impl ops::Add<&Vec3> for Vec3 {
  type Output = Vec3;
  fn add(self, rhs: &Vec3) -> Vec3 {
    return &self + rhs;
  }
}

impl ops::Add<Vec3> for Vec3 {
  type Output = Vec3;
  fn add(self, rhs: Vec3) -> Vec3 {
    return &self + &rhs;
  }
}

impl ops::Sub<&Vec3> for Vec3 {
  type Output = Vec3;
  fn sub(self, rhs: &Vec3) -> Vec3 {
    return &self + rhs;
  }
}

impl ops::Sub<Vec3> for Vec3 {
  type Output = Vec3;
  fn sub(self, rhs: Vec3) -> Vec3 {
    return &self + &rhs;
  }
}

impl ops::Sub<&Vec3> for &Vec3 {
  type Output = Vec3;
  fn sub(self, rhs: &Vec3) -> Vec3 {
    Vec3 {
      position: [self.position[0] - rhs.position[0], self.position[1] - rhs.position[1], self.position[2] - rhs.position[2]],
      rgb:  [self.rgb[0] - rhs.rgb[0], self.rgb[1] - rhs.rgb[1], self.rgb[2] - rhs.rgb[2]],
    }
  }
}

impl ops::Div<Vec3> for Vec3 {
  type Output = Vec3;
  fn div(self, rhs: Vec3) -> Vec3 {
    return &self/&rhs;
  }
}

impl ops::Div<&Vec3> for Vec3 {
  type Output = Vec3;
  fn div(self, rhs: &Vec3) -> Vec3 {
    return &self/rhs;
  }
}

impl ops::Div<&Vec3> for &Vec3 {
  type Output = Vec3;
  fn div(self, rhs: &Vec3) -> Vec3 {
    Vec3 {
      position: [self.position[0] / rhs.position[0], self.position[1] / rhs.position[1], self.position[2] / rhs.position[2]],
      rgb:  [self.rgb[0] / rhs.rgb[0], self.rgb[1] / rhs.rgb[1], self.rgb[2] / rhs.rgb[2]],
    }
  }
}

impl ops::Div<f32> for &Vec3 {
  type Output = Vec3;
  fn div(self, rhs: f32) -> Vec3 {
    Vec3 {
      position: [self.position[0] / rhs, self.position[1] / rhs, self.position[2] / rhs],
      rgb:  self.rgb,
    }
  }
}

impl ops::Div<f32> for Vec3 {
  type Output = Vec3;
  fn div(self, rhs: f32) -> Vec3 {
    return &self/rhs;
  }
}

impl ops::Mul<&Vec3> for &Vec3 {
  type Output = Vec3;
  fn mul(self, rhs: &Vec3) -> Vec3 {
    Vec3 {
      position: [self.position[0] * rhs.position[1], self.position[1] * rhs.position[1], self.position[2] * rhs.position[2]],
      rgb: [self.rgb[0] * rhs.rgb[0], self.rgb[1] * rhs.rgb[1], self.rgb[2] * rhs.rgb[2]],
    }
  }
}

impl ops::Mul<Vec3> for Vec3 {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Vec3 {
    return &self*&rhs;
  }
}

impl ops::Mul<&Vec3> for Vec3 {
  type Output = Vec3;
  fn mul(self, rhs: &Vec3) -> Vec3 {
    return &self*rhs;
  }
}

impl ops::Mul<f32> for &Vec3 {
  type Output = Vec3;
  fn mul(self, rhs: f32) -> Vec3 {
    Vec3 {
      position: [self.position[0]* rhs, self.position[1] * rhs, self.position[2] * rhs],
      rgb: self.rgb,
    }
  }
}

impl ops::Mul<f32> for Vec3 {
  type Output = Vec3;
  fn mul(self, rhs: f32) -> Vec3 {
    return &self*rhs;
  }
}

impl ops::AddAssign for Vec3 {
  fn add_assign(&mut self, rhs: Self) {
    *self = self.clone()+rhs;
  }
}

impl ops::SubAssign for Vec3 {
  fn sub_assign(&mut self, rhs: Self) {
    *self = self.clone() - rhs;
  }
}

impl ops::MulAssign for Vec3 {
  fn mul_assign(&mut self, rhs: Self) {
    *self = self.clone() * rhs;
  }
}

impl ops::DivAssign for Vec3 {
  fn div_assign(&mut self, rhs: Self) {
    *self = self.clone() / rhs;
  }
}

fn main() {
  let nx = 200;
  let ny = 100;
  print!("P3\n{} {}\n255\n", nx, ny);
  for j in (0..ny).rev() {
    for i in 0..nx {
      let vector = Vec3::new(Vec3Init::PositionParameter(i as f32 / nx as f32, j as f32 / ny as f32, 0.2));
      let ir = (255.99 * vector.x()).round() as i32;
      let ig = (255.99 * vector.y()).round() as i32;
      let ib = (255.99 * vector.z()).round() as i32;
      print!("{} {} {}\n", ir, ig, ib);
    }
  }
}