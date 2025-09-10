// define a struct for a "Computer"
pub struct Computer {
    cpu: String,
    ram: u32,       
    storage: u32,   
    gpu: Option<String>,
}

impl Computer {
    pub fn show_specs(&self) {
        println!("Computer Specs:");
        println!("CPU: {}", self.cpu);
        println!("RAM: {} GB", self.ram);
        println!("Storage: {} GB", self.storage);
        match &self.gpu {
            Some(g) => println!("GPU: {}", g),
            None => println!("GPU: None"),
        }
    }
}

// builder struct
pub struct ComputerBuilder {
    cpu: String,
    ram: u32,
    storage: u32,
    gpu: Option<String>,
}

impl ComputerBuilder {
    // start a new builder with defaults
    pub fn new() -> Self {
        Self {
            cpu: "Unknown".to_string(),
            ram: 4,
            storage: 256,
            gpu: None,
        }
    }

    pub fn cpu(mut self, cpu: &str) -> Self {
        self.cpu = cpu.to_string();
        self
    }

    pub fn ram(mut self, ram: u32) -> Self {
        self.ram = ram;
        self
    }

    pub fn storage(mut self, storage: u32) -> Self {
        self.storage = storage;
        self
    }

    pub fn gpu(mut self, gpu: &str) -> Self {
        self.gpu = Some(gpu.to_string());
        self
    }

    pub fn build(self) -> Computer {
        Computer {
            cpu: self.cpu,
            ram: self.ram,
            storage: self.storage,
            gpu: self.gpu,
        }
    }
}

// runner for computer builder example
pub fn run() {
    let gaming_pc = ComputerBuilder::new()
        .cpu("Intel i9")
        .ram(32)
        .storage(1024)
        .gpu("NVIDIA RTX 4090")
        .build();

    gaming_pc.show_specs();

    println!("----------------------");

    let office_pc = ComputerBuilder::new()
        .cpu("AMD Ryzen 5")
        .ram(16)
        .storage(512)
        .build();

    office_pc.show_specs();

    println!("----------------------");
}
