use crate::chromosome::Chromosome;


pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub struct TestIndividual {
    fitness: f32,
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }

    pub fn chromosome(&self) -> &Chromosome {
        todo!()
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }
    fn chromosome(&self) -> &Chromosome {
        panic!("not supported for TestIndividual")
    }
}
