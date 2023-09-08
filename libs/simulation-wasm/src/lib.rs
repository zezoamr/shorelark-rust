use lib_simulation as sim;
use rand::prelude::*;
use wasm_bindgen::prelude::*;
use serde::Serialize;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }
    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        serde_wasm_bindgen::to_value(&world).unwrap()
    }
    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }
}


#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub animals: Vec<Animal>,
    pub foods: Vec<Food>,
}


#[derive(Clone, Debug, Serialize)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

// ^ This model is smaller than `lib_simulation::Animal` - that's
// | because a bird's position is all we need on the JavaScript's
// | side at the moment; there's no need to map rest of the fields.

#[derive(Clone, Debug, Serialize)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}


impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world
            .animals()
            .iter()
            .map(Animal::from)
            .collect();

        let foods = world
            .foods()
            .iter()
            .map(Food::from)
            .collect();

        Self { animals, foods }
    }
}


impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
            rotation: animal.rotation().angle(),
        }
    }
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}

// to build wasm-pack build