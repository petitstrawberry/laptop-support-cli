use laptop_support::laptop::Laptop;

pub trait CommandTrait {
    fn execute(&self, laptop: &Laptop);
}
