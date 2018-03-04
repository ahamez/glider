pub mod glider;
use glider::universe::Universe;

fn main() {

  let u = Universe::new(10, 10);
  u.tick().tick();
}
