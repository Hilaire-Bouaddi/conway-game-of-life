
pub struct Cell 
{
    x: usize,
    y: usize,
    alive: bool
}

impl Cell 
{
    pub fn new(x: usize, y: usize) -> Cell
    {
        Self {
            x: x,
            y: y,
            alive: false
        }
    }

    pub fn get_x(&self) -> usize
    {
        self.x
    }

    pub fn get_y(&self) -> usize
    {
        self.y
    }

    pub fn is_alive(&self) -> bool
    {
        self.alive
    }
    pub fn set_alive(&mut self, alive: bool) 
    {
        self.alive = alive
    }
}