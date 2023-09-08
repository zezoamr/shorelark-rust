use nalgebra as na;
use rand::{RngCore, Rng};
use std::f32::consts::*;
pub use self::{animal::*, brain::*, eye::*, food::*, animal_individual::*};
use lib_neural_network as nn;
use lib_genetic_algorithm as ga;

mod animal_individual;
mod brain;
mod animal;
mod food;
mod eye;

const GENERATION_LENGTH: usize = 2500;
pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
}

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}


// FRAC_PI_2 = PI / 2.0; a convenient shortcut
use std::f32::consts::FRAC_PI_2;
const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);

        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::new(),
            ga::UniformCrossover::new(),
            ga::GaussianMutation::new(0.01, 0.3),
        );

        Self { world, ga, age: 0 }
    }
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            self.evolve(rng);
        }
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
                    animal.satiation += 1;
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision = animal.eye.process_vision(
                animal.position,
                animal.rotation,
                &self.world.foods,
            );

            let response = animal.brain.nn.propagate(vision);

            let speed = response[0].clamp(
                -SPEED_ACCEL,
                SPEED_ACCEL,
            );

            let rotation = response[1].clamp(
                -ROTATION_ACCEL,
                ROTATION_ACCEL,
            );

            animal.speed =
                (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);

            animal.rotation = na::Rotation2::new(
                animal.rotation.angle() + rotation,
            );

            // (btw, there is no need for ROTATION_MIN or ROTATION_MAX,
            // because rotation automatically wraps from 2*PI back to 0 
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;
    
        // Transforms `Vec<Animal>` to `Vec<AnimalIndividual>`
        let mut current_population: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();
    
        // Evolves this `Vec<AnimalIndividual>`
        let evolved_population = self.ga.evolve(
            rng,
            &mut current_population,
        );
    
        // Transforms `Vec<AnimalIndividual>` back into `Vec<Animal>`
        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();
    
        for food in &mut self.world.foods {
            food.position = rng.gen();
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