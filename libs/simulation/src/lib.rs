use nalgebra as na;
use rand::{RngCore, Rng};
use std::f32::consts::*;

pub struct Simulation {
    world: World,
}

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movements();
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed); 
            //rotating relative to the y axis, that is: a bird with rotation of 0° will fly upwards. 

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0); 
            //Our map is bounded by <0.0, 1.0>, anything outside wouldd be rendered outside the canvas
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        // treating both as spheres
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(
                    &animal.position,
                    &food.position,
                );
    
                if distance <= 0.01 {
                    food.position = rng.gen();
                }
            }
        }
    }

}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40)
            .map(|_| Animal::random(rng))
            .collect();

        let foods = (0..60)
            .map(|_| Food::random(rng))
            .collect();

        // | A more complex solution could be based off of e.g.
        // | Poisson disk sampling:
        

        Self { animals, foods }
    }
    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

#[derive(Debug)]
pub struct Animal {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32,
}

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f32>,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),

            rotation: rng.gen(),
            speed: 0.002,
        }
    }
    pub fn position(&self) -> na::Point2<f32> {
        // ------------------ ^
        // | No need to return a reference, because na::Point2 is Copy.
        // |
        // | (meaning: it's so small that cloning it is cheaper than
        // | messing with references.)

        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}

const FOV_RANGE: f32 = 0.25;
const FOV_ANGLE: f32 = PI + FRAC_PI_4;

/// How much photoreceptors there are in a single eye.
///
/// More cells means our birds will have more "crisp" vision, allowing
/// them to locate the food more precisely - but the trade-off is that
/// the evolution process will then take longer, or even fail, unable
/// to find any solution.
///
/// I've found values between 3~11 sufficient, with eyes having more
/// than ~20 photoreceptors yielding progressively worse results.
const CELLS: usize = 9;


#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye {
    // FOV_RANGE, FOV_ANGLE & CELLS are the values we'll use during
    // simulation - but being able to create an arbitrary eye will
    // come handy during the testing:
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self { fov_range, fov_angle, cells }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            let vec = food.position - position;
            let dist = vec.norm();

            if dist > self.fov_range {
                continue;
            }
            // Returns vector's direction relative to the X axis
            let angle = na::Rotation2::rotation_between(&na::Vector2::y(), &vec).angle();
            // Because our bird is *also* rotated, we have to include its
            // rotation too:
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);

            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            // Makes angle *relative* to our birdie's field of view - that is:
            // transforms it from <-FOV_ANGLE/2,+FOV_ANGLE/2> to <0,FOV_ANGLE>.
            // - an angle of 0° means "the beginning of the FOV",
            let angle = angle + self.fov_angle / 2.0;
            
            // Since this angle is now in range <0,FOV_ANGLE>, by dividing it by
            // FOV_ANGLE, we transform it to range <0,1>. The value we get can be treated as a percentage
            let cell = angle / self.fov_angle;
            
            // With cell in range <0,1>, by multiplying it by the number of
            // cells we get range <0,CELLS> - this corresponds to the actual
            // cell index inside our `cells` array.
            let cell = cell * (self.cells as f32);

            // Our `cell` is of type `f32` - before we're able to use it to index an array, we have to convert it to `usize`.
            // We're also doing `.min()` to cover an extreme edge case: for
            // cell=1.0 (which corresponds to a food being maximally to the right side of our birdie), we'd get `cell` of `cells.len()`,
            // which is one element *beyond* what the `cells` array contains (its range is <0, cells.len()-1>).
            let cell = (cell as usize).min(cells.len() - 1);
            
            // Energy is inversely proportional to the distance between our
            // birdie and the currently checked food; that is - an energy of:
            // - 0.0001 = food is barely in the field of view (i.e. far away),
            // - 1.0000 = food is right in front of the bird.
            let energy = (self.fov_range - dist) / self.fov_range;

            cells[cell] += energy;
        }

        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

impl Eye {
    fn new_ex(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EYE_CELLS: usize = 13;

    fn food(x: f32, y: f32) -> Food {
        Food {
            position: na::Point2::new(x, y),
        }
    }

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new_ex(self.fov_range, self.fov_angle, TEST_EYE_CELLS);

            let actual = eye.process_vision(
                na::Point2::new(self.x, self.y),
                na::Rotation2::new(self.rot),
                &self.foods,
            );

            let actual = actual
                .into_iter()
                .map(|cell| {
                    if cell >= 0.7 {
                        "#"
                    } else if cell >= 0.3 {
                        "+"
                    } else if cell > 0.0 {
                        "."
                    } else {
                        " "
                    }
                })
                .collect::<Vec<_>>()
                .join("");

            assert_eq!(actual, self.expected);
        }
    }

    mod different_fov_ranges {
        use super::*;
        use test_case::test_case;

        #[test_case(1.0, "      +      ")] // Food is inside the FOV
        #[test_case(0.9, "      +      ")] // ditto
        #[test_case(0.8, "      +      ")] // ditto
        #[test_case(0.7, "      .      ")] // Food slowly disappears
        #[test_case(0.6, "      .      ")] // ditto
        #[test_case(0.5, "             ")] // Food disappeared!
        #[test_case(0.4, "             ")]
        #[test_case(0.3, "             ")]
        #[test_case(0.2, "             ")]
        #[test_case(0.1, "             ")]
        fn test(fov_range: f32, expected: &'static str) {
            TestCase {
                foods: vec![food(0.5, 1.0)],
                fov_angle: FRAC_PI_2,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_range,
                expected,
            }
            .run()
        }
    }

    mod different_fov_angles {
        use super::*;
        use test_case::test_case;

        #[test_case(0.25 * PI, " +         + ")] // FOV is narrow = 2 foods
        #[test_case(0.50 * PI, ".  +     +  .")]
        #[test_case(0.75 * PI, "  . +   + .  ")] // FOV gets progressively
        #[test_case(1.00 * PI, "   . + + .   ")] // wider and wider...
        #[test_case(1.25 * PI, "   . + + .   ")]
        #[test_case(1.50 * PI, ".   .+ +.   .")]
        #[test_case(1.75 * PI, ".   .+ +.   .")]
        #[test_case(2.00 * PI, "+.  .+ +.  .+")] // FOV is wide = 8 foods
        fn test(fov_angle: f32, expected: &'static str) {
            TestCase {
                foods: vec![
                    food(0.0, 0.0),
                    food(0.0, 0.33),
                    food(0.0, 0.66),
                    food(0.0, 1.0),
                    food(1.0, 0.0),
                    food(1.0, 0.33),
                    food(1.0, 0.66),
                    food(1.0, 1.0),
                ],
                fov_range: 1.0,
                x: 0.5,
                y: 0.5,
                rot: 3.0 * FRAC_PI_2,
                fov_angle,
                expected,
            }
            .run()
        }
    }

    mod different_positions {
        use super::*;
        use test_case::test_case;

        // Checking the X axis:
        // (you can see the bird is "flying alongside" the foods)
        #[test_case(0.9, 0.5, "#           #")]
        #[test_case(0.8, 0.5, "  #       #  ")]
        #[test_case(0.7, 0.5, "   +     +   ")]
        #[test_case(0.6, 0.5, "    +   +    ")]
        #[test_case(0.5, 0.5, "    +   +    ")]
        #[test_case(0.4, 0.5, "     + +     ")]
        #[test_case(0.3, 0.5, "     . .     ")]
        #[test_case(0.2, 0.5, "     . .     ")]
        #[test_case(0.1, 0.5, "     . .     ")]
        #[test_case(0.0, 0.5, "             ")]
        //
        // Checking the Y axis:
        // (you can see the bird is "flying alongside" the foods)
        #[test_case(0.5, 0.0, "            +")]
        #[test_case(0.5, 0.1, "          + .")]
        #[test_case(0.5, 0.2, "         +  +")]
        #[test_case(0.5, 0.3, "        + +  ")]
        #[test_case(0.5, 0.4, "      +  +   ")]
        #[test_case(0.5, 0.6, "   +  +      ")]
        #[test_case(0.5, 0.7, "  + +        ")]
        #[test_case(0.5, 0.8, "+  +         ")]
        #[test_case(0.5, 0.9, ". +          ")]
        #[test_case(0.5, 1.0, "+            ")]
        fn test(x: f32, y: f32, expected: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
                fov_range: 1.0,
                fov_angle: FRAC_PI_2,
                rot: 3.0 * FRAC_PI_2,
                x,
                y,
                expected,
            }
            .run()
        }
    }

    mod different_rotations {
        use super::*;
        use test_case::test_case;

        #[test_case(0.00 * PI, "         +   ")] // Food is to our right
        #[test_case(0.25 * PI, "        +    ")]
        #[test_case(0.50 * PI, "      +      ")] // Food is in front of us
        #[test_case(0.75 * PI, "    +        ")]
        #[test_case(1.00 * PI, "   +         ")] // Food is to our left
        #[test_case(1.25 * PI, " +           ")]
        #[test_case(1.50 * PI, "            +")] // Food is behind us
        #[test_case(1.75 * PI, "           + ")] // (we continue to see it
        #[test_case(2.00 * PI, "         +   ")] // due to 360° fov_angle.)
        #[test_case(2.25 * PI, "        +    ")]
        #[test_case(2.50 * PI, "      +      ")]
        fn test(rot: f32, expected: &'static str) {
            TestCase {
                foods: vec![food(0.0, 0.5)],
                fov_range: 1.0,
                fov_angle: 2.0 * PI,
                x: 0.5,
                y: 0.5,
                rot,
                expected,
            }
            .run()
        }
    }
}