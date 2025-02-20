// NOTE: push_back used. Must confirm if allowed, and what other methods can be used if it isn't

use self::LinkedList::*;
use im::list::*;
use im::List;

#[derive(Debug, PartialEq)]
pub enum LinkedList<T> {
    Tail,
    Head(List<T>),
}

impl<T> LinkedList<T> {
    // returns empty linked list
	pub fn empty() -> Self {
		LinkedList::Tail
	}

	// returns created linked list with 1 head
	pub fn new(t: T) -> Self {
		LinkedList::Head(cons(t, List::new()))
	}

	// push item into front of linked list
	pub fn push(&self, value: T) -> Self {
		match self {
			// if self is a tail, create new linked list
			LinkedList::Tail => {
				Self::new(value)
			}
			// if self is a head, create value as first item
			// and append it to the front of the rest of the list
			LinkedList::Head(list) => {
				LinkedList::Head(cons(value, list))
			}
		}
	}

	// push item into back of linked list
	pub fn push_back(&mut self, value: T) {
		match self {
			// if self is a Tail, turn it into a Head with the value
			LinkedList::Tail => {
				*self = Self::new(value);
			}
			// if self is a Head
			LinkedList::Head(list) => {
                *self = Head(list.push_back(value));
			}
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut l = LinkedList::new(3);
        l = l.push(4);
        assert_eq!(l,Head(cons(4, cons(3, List::new()))));

        l.push_back(2);
        assert_eq!(l,Head(cons(4, cons(3, cons(2, List::new())))));
    }
}

fn main() {
    // Construct a list: [1, 2]
    // let list = list::cons(1, list::cons(2, List::new()));

    // // Print the list
    // println!("{:?}", list); // Output: List [1, 2]
}
