use network::Network;

pub struct Slab {
    pub id: u32
}

impl Slab {
    pub fn new(net : &Network) -> Slab {
        Slab {
            id: net.generate_slab_id()
        }
    }
}
