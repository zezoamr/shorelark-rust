use crate::chromosome::Chromosome;

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome) -> Self;
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub enum TestIndividual {
    // WithChromosome and WithFitness are two variants of the TestIndividual enum
    /// For tests that require access to chromosome
    WithChromosome { chromosome: Chromosome },

    /// For tests that don't require access to chromosome
    WithFitness { fitness: f32 },
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self::WithFitness { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn create(chromosome: Chromosome) -> Self {
        Self::WithChromosome { chromosome }
    }

    fn chromosome(&self) -> &Chromosome {
        match self {
            Self::WithChromosome { chromosome } => chromosome,

            Self::WithFitness { .. } => {
                panic!("not supported for TestIndividual::WithFitness")
            }
        }
    }

    fn fitness(&self) -> f32 {
        // Match on self to determine which variant of TestIndividual we're dealing with
        match self {
            // If self is a WithChromosome variant
            Self::WithChromosome { chromosome } => {
                // Calculate the fitness value by summing up all the values in its chromosome field
                chromosome.iter().sum()
            }

            // If self is a WithFitness variant
            Self::WithFitness { fitness } => {
                // Return the value of its fitness field
                *fitness
            }
        }
    }
}

