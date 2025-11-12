use std::io;

#[derive(Debug, Clone)]
struct PDA {
    address: String,
    buff: String,
    sender: String,
    receiver: String,
}

impl PDA {
    fn new(id: u32) -> Self {
        Self {
            address: format!("PDA-{}", id),
            buff: String::new(),
            sender: String::new(),
            receiver: String::new(),
        }
    }

    fn clear(&mut self) {
        self.buff.clear();
        self.sender.clear();
        self.receiver.clear();
        println!("{} cleared!", self.address);
    }

    fn update(&mut self, sender: &str, receiver: &str, buff: &str) {
        self.sender = sender.to_string();
        self.receiver = receiver.to_string();
        self.buff = buff.to_string();
    }
}

struct RingManager {
    pdas: Vec<PDA>,
    current_index: usize,
}

impl RingManager {
    fn new(pda_count: u32) -> Self {
        let pdas = (0..pda_count).map(|i| PDA::new(i)).collect();
        Self {
            pdas,
            current_index: 0,
        }
    }

    fn add_entry(&mut self, buff: &str, sender: &str, receiver: &str) {
        let total = self.pdas.len();

        // شماره PDA فعلی برای نوشتن
        let write_index = self.current_index % total;

        // شماره PDA که باید پاک بشه (دو تا قبل‌تر)
        let clear_index = (self.current_index + total - 2) % total;

        // بنویس داخل PDA فعلی
        self.pdas[write_index].update(buff, sender, receiver);
        println!("✍️ Updated {}", self.pdas[write_index].address);

        // اگر حداقل از PDA سوم به بعدی هستیم، پاک کن دو تا قبل‌تر رو
        if self.current_index >= 2 {
            self.pdas[clear_index].clear();
            println!("🧹 Cleared {}", self.pdas[clear_index].address);
        }

        // برو به بعدی
        self.current_index = (self.current_index + 1) % total;
    }

    fn show_all(&self) {
        println!("\n📦 Current PDA States:");
        for pda in &self.pdas {
            println!("{:?}", pda);
        }
        println!("----------------------------\n");
    }
}

fn main() {
    let mut ring = RingManager::new(5); // ۵ تا PDA
    println!("PDA cicle started. Type input (or 'exit'):\n");

    loop {
        let mut sender = String::new();
        let mut receiver = String::new();
        let mut buff = String::new();

        println!("Enter sender:");
        io::stdin().read_line(&mut sender).unwrap();

        println!("Enter receiver:");
        io::stdin().read_line(&mut receiver).unwrap();

        println!("Enter message:");
        io::stdin().read_line(&mut buff).unwrap();

        let sender = sender.trim();
        let receiver = receiver.trim();
        let buff = buff.trim();

        if sender.eq_ignore_ascii_case("exit")
            || receiver.eq_ignore_ascii_case("exit")
            || buff.eq_ignore_ascii_case("exit")
        {
            println!("👋 Exiting...");
            break;
        }

        ring.add_entry(sender, receiver, buff);
        ring.show_all();
    }
}
