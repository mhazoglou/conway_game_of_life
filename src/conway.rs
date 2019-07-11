use std::{cell::RefCell, fmt};
use rand::Rng;

pub fn calc_idx_periodic(hor_idx: u32, vert_idx: u32, 
                     width: u32, height: u32) -> usize {
    ((hor_idx % width) + width * (vert_idx % height)) as usize
}

//#[derive(Debug)]
pub struct Grid{
    // RefCell to get mutability in a generally
    // immutable attribute
    refstate: RefCell<Vec<u8>>,
    refstep: RefCell<u32>,
    pub width: u32,
    pub height: u32,
}

impl Grid{
    pub fn new(width: u32, height: u32) -> Grid {
        if width == 0 || height == 0 {
            panic!("Width and/or height of rectangular grid cannot be zero.");
        }
        let tot_nodes: u32 = width * height;
        let mut rng = rand::thread_rng();
/*         // functional approach
        let state: Vec<u8> = (0..tot_nodes).map(|_| {
            rng.gen_range(0, 2) // 0 included, 2 excluded
        }).collect();*/
        
        
        // imperative approach
        let mut state: Vec<u8> = 
            Vec::with_capacity(tot_nodes as usize);
        for _ in 0..tot_nodes {
            state.push(rng.gen_range(0, 2));
        }
        
        /*
        // procedural approach
        let mut state: Vec<u8> = vec![0; tot_nodes as usize];
            //Vec::with_capacity(tot_nodes as usize);
        for i in &mut state {
            *i = rng.gen_range(0, 2);
        }
        */
        
        Grid {
            width,
            height,
            refstate: RefCell::new(state),
            refstep: RefCell::new(0),
        }
    }
    
    pub fn new_like(other_grid: &Grid) -> Grid {
        let tot_nodes: u32 = other_grid.width * 
                                other_grid.height;
        let mut rng = rand::thread_rng();
        // functional approach
        let state: Vec<u8> = (0..tot_nodes).map(|_| {
            rng.gen_range(0, 2) // 0 included, 2 excluded
        }).collect();
        Grid {
            refstate: RefCell::new(state),
            refstep: RefCell::new(0),
            ..*other_grid
        }
    }
    
    pub fn reset(&self) {
        let mut step = self.refstep.borrow_mut();
        *step = 0;
        
        let mut state = self.refstate.borrow_mut();
        let mut rng = rand::thread_rng();
        for i in state.iter_mut() {
            *i = rng.gen_range(0, 2);
        }
    }
    
    pub fn evolve(&self) {
        let mut step = self.refstep.borrow_mut();
        *step += 1;
        
        let mut state = self.refstate.borrow_mut();
        let state_copy = state.clone();
        for i in 0..self.height {
            for j in 0..self.width{
                let idx = calc_idx_periodic(j, i, 
                                            self.width,
                                            self.height
                                           );
                let hor_shift = [1, 0, self.width - 1];
                let vert_shift = [1, 0, self.height - 1];
                let mut nn_count = 0;
                // summing over nearest neighbors
                for hor_sh in hor_shift.iter(){
                    for vert_sh in vert_shift.iter() {
                        if *vert_sh != 0 || *hor_sh != 0 {
                            nn_count += state_copy[
                                calc_idx_periodic(j + hor_sh,
                                                  i + vert_sh,
                                                  self.width,
                                                  self.height
                                                 )];
                        }
                    }
                }
                if nn_count < 2 || nn_count > 3 {
                    state[idx] = 0
                } else if nn_count == 3 {
                    state[idx] = 1
                }
                // if nn_count == 2 keep the state as is
            }
        }
    }
    
    pub fn print_state(&self) {
        let state = self.refstate.borrow();
        let mut row: Vec<u8> = vec![0; self.width as usize];
        for i in 0..self.height {
            for (j, col) in row.iter_mut().enumerate() {
                let idx = calc_idx_periodic(j as u32, i, 
                                            self.width,
                                            self.height
                                           );
                *col = state[idx];
            }
            println!("{:?}", row);
        }
    }
    
    pub fn set_state(&self, new_state: &[u8]) {
        let len_new = new_state.len();
        let tot_nodes = (self.width * self.height) as usize;
        if tot_nodes != len_new {
            panic!(
            "The size of the input state is not the same as the width times height"
            );
        }
        
        for el in new_state {
            if *el > 1 {
                panic!("Valid values can only be 0 or 1. Got value of {}.", *el);
            }
        }
        
        let mut state = self.refstate.borrow_mut();
        *state = new_state.to_vec();
    }
    
    pub fn eat_state(&self, new_state: Vec<u8>) {
        let len_new = new_state.len();
        let tot_nodes = (self.width * self.height) as usize;
        if tot_nodes != len_new {
            panic!(
            "The size of the input state is not the same as the width times height"
            );
        }
        
        for el in &new_state {
            if *el > 1 {
                panic!("Valid values can only be 0 or 1. Got value of {}.", *el);
            }
        }
        
        let mut state = self.refstate.borrow_mut();
        *state = new_state;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // lucky me returning Result can use ? operator
        let step = self.refstep.borrow();
        write!(f, 
               "Grid type: {{\n\n    width: {}, height: {}, step: {}\n", 
               self.width, self.height, step)?;
        let state = self.refstate.borrow();
        let mut row: Vec<u8> = vec![0; self.width as usize];
        for i in 0..self.height {
            write!(f, "{}|\n", 
                   &"|\u{2014}\u{2014}\u{2014}".repeat(self.width as usize)[..]
                   )?;
            //write!(f, "[")?;
            for (j, col) in row.iter_mut().enumerate() {
                let idx = calc_idx_periodic(j as u32, i, 
                                            self.width,
                                            self.height
                                           );
                *col = state[idx];
                if *col == 1 {
                    write!(f, "| \u{25A0} ")?;
                } else {
                    write!(f, "|   ")?;
                }
            }
            write!(f, "|\n")?;
            // another unnecessary assignment to make warnings stop
            
            //write!(f, "{:?}\n", row)?;
        }
        write!(f, "{}|\n\n}}\n", &"|\u{2014}\u{2014}\u{2014}".repeat(self.width as usize)[..])
        // getting an annoying warning for no reason
    }
}