use self::LinkedList::*;

#[derive(Debug, PartialEq, Clone)]
pub enum LinkedList<T> {
	Tail,
	Head(T, Box<LinkedList<T>>),
}

impl<T: Clone> LinkedList<T> {
	// returns empty linked list
	pub fn empty() -> Self {
		LinkedList::Tail
	}

	// returns created linked list with 1 head
	pub fn new(t: T) -> Self {
		LinkedList::Head(t, Box::new(Self::empty()))
	}

	// push item into front of linked list
	pub fn push(&self, value: T) -> Self {
		match self {
			// if self is a tail, create new linked list
			LinkedList::Tail => {
				Self::new(value)
			}
			// if self is a head, create value as first item
			// and have it point to rest of the list
			LinkedList::Head(_, _) => {
				LinkedList::Head(value, Box::new(self.clone()))
			}
		}
	}

	// push item into back of linked list
	pub fn push_back(&mut self, value: T) {
		match self {
			// if self is a tail, turn it into a Head with the value
			LinkedList::Tail => {
				*self = Self::new(value)
			}
			// if self is a head, go to next item (Head or Tail)
			LinkedList::Head(_, next) => {
				next.push_back(value);
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
		assert_eq!(l,Head(4,Box::new(Head(3,Box::new(Tail)))));

		l.push_back(2);
		assert_eq!(l,Head(4,Box::new(Head(3,Box::new(Head(2,Box::new(Tail)))))));


	}
}

fn main() {
}