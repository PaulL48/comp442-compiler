use std::fmt;

pub struct RegisterPool {
    next_available: usize,
}

pub struct RegisterRental {
    n: usize,
}

pub struct Register(usize);

pub const R0: Register = Register(0);
pub const R1: Register = Register(1);
pub const R13: Register = Register(13);
pub const R14: Register = Register(14);
pub const R15: Register = Register(15);

impl RegisterPool {
    const MAX: usize = 12;

    pub fn register(i: usize) -> String {
        format!("r{}", i)
    }

    pub fn new() -> Self {
        RegisterPool { next_available: 1 }
    }

    pub fn reserve(&self, n: usize) -> RegisterRental {
        RegisterRental { n }
    }

    pub fn release(&mut self, rental: RegisterRental) {
        for _ in 0..rental.n {
            self.push()
        }
    }

    pub fn pop(&mut self) -> Register {
        if self.next_available == RegisterPool::MAX {
            panic!("Requesting more registers than available");
        }

        let result = self.next_available;
        self.next_available += 1;
        Register(result)
    }

    fn push(&mut self) {
        if self.next_available == 0 {
            panic!("Returning more registers than available");
        }

        self.next_available -= 1;
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", RegisterPool::register(self.0))
    }
}
