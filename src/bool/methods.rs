pub trait Toggle {
    fn toggle(&mut self);
}


impl Toggle for bool {
    fn toggle(&mut self) {
        match self {
            true => *self = false,
            false => *self = true
        }
    }
}
