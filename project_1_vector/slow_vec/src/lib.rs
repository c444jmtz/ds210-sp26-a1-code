use std::fmt::{Display, Formatter};
use fixed::FixedSizeArray;

// A SlowVec contains a fixed number of elements.
// The elements are of type "T"
// This is Rust's way of saying that SlowVec can accept any type for the elements.
// E.g., SlowVec<i32> represents a SlowVec with integer elements,
//       SlowVec<bool> represents a SlowVec with bool elements,
//       etc.
// look at main.rs for an example.
pub struct SlowVec<T> {
    fixed: FixedSizeArray<T>,
}

// Functions inside SlowVec.
impl<T> SlowVec<T> {
    pub fn new() -> Self {
        
        return SlowVec {
            
            fixed: FixedSizeArray::allocate(0)
        };
    }
    
    // returns the length of the SlowVec.
    pub fn len(&self) -> usize {
       
        return self.fixed.len();
    }

    // Transforms an instance of SlowVec to a regular vector.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.fixed.len());
        for i in 0..self.fixed.len() {
            v.push(self.fixed.move_out(i));
        }
        v
    }

    // Transforms a vector to a SlowVec.
    pub fn from_vec(vec: Vec<T>) -> SlowVec<T> {
        let mut tmp = FixedSizeArray::allocate(vec.len());
        let mut index = 0;
        for element in vec {
            tmp.put(element, index);
            index = index + 1;
        }
        return SlowVec { fixed: tmp };
    }

    // Clear the content of this vector.
    pub fn clear(&mut self) {
        self.fixed = FixedSizeArray::allocate(0);
    }

    // Get a reference to the element at position i.
    // Think of a reference as a read-only "copy" of the element.
    // We will talk about what references are in class.
    // Note: the element remains stored in the SlowVec after get(). It is not removed.
    pub fn get(&self, i: usize) -> &T {
        self.fixed.get(i)
    }

    // Student 1: Provide your solution here. (Christopher Martinez)
    pub fn push(&mut self, t: T) {
    // Create new fixed array with an extra slot to make space for value
   
       let mut new_fixed = FixedSizeArray::allocate(self.len() + 1);

    // iterate through each value in the old array, and clone into new array
     // .get(i).clone() because get() returns &T (sliced), and we need an owned T. (Used ai for this part)


    for i in 0..self.len() {
        //put expects T, push provides &T
        new_fixed.put(self.fixed.get(i).clone(), i);
    }
// add new element at the end of array
    new_fixed.put(t, self.len());
//replace old array to the new array with new value
    self.fixed = new_fixed;
}

    }

    // Student 2: Provide your solution here
    pub fn remove(&mut self, i: usize) {
        todo!("Student 2 should implement this");
    }
}


// This allows us to print the SlowVec using println!().
impl<T: Display> Display for SlowVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SlowVec({})", self.fixed)
    }
}
