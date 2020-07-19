pub struct MemoryModule {}

impl MemoryModule {
    pub fn get_memory_size(name: &str) -> usize {
        match name {
            "basic" => 20,
            "plus" => 30,
            "ikito" => 40,
            "jindai" => 80,
            _ => 20,
        }
    }
}
