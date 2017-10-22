use std::collections::LinkedList;

struct Tower {
    name: String,
    rings: LinkedList<Ring>
}

impl Tower {
    fn new(name: &str) -> Self {
        Tower {
            name: String::from(name),
            rings: LinkedList::new()
        }
    }

    fn pop(&mut self) -> Ring {
        self.rings.pop_front().unwrap()
    }

    fn push(&mut self, ring: Ring) {
        let next = match self.rings.iter().next() {
            Some(r) => r.size,
            None    => 0
        };

        if next > ring.size {
            panic!("Cannot push {} onto tower {}, last value in tower is {}", ring.size,
                self.name, next);
        }

        self.rings.push_front(ring);
    }

    fn size(&self) -> u8 {
        self.rings.len() as u8
    }
}

impl std::fmt::Debug for Tower {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Tower {}: {:?}", self.name, self.rings)
    }
}

struct Ring {
    size: u8
}

impl std::fmt::Debug for Ring {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Ring({})", self.size)
    }
}

impl Ring {
    fn new(size: u8) -> Self {
        Ring { size }
    }
}

fn hanoi() {
    let (mut a, mut b, mut c) = (Tower::new("A"), Tower::new("B"), Tower::new("C"));

    // load the rings
    for s in 1..4 {
        a.rings.push_back(Ring::new(s));
    }
}

fn main() {
    hanoi();
}
