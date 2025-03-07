use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    chs: HashMap<char, TrieNode>,
    value: Option<i32>,
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

fn split_first(s: &String) -> Option<(char, String)> {
    let mut chars = s.chars(); // Create an iterator over chars
    chars.next().map(|first| (first, chars.collect())) // Extract first char and collect the rest
}

impl TrieNode {
    // get number of nodes children+self that have a value
    fn get_num_children_and_self(&self) -> usize {
        let mut num_children: usize = 0;

        // if self is a value include self
        if self.value.is_some() {
            num_children += 1;
        }

        // get the hashmap
        let hashmap = &self.chs;

        // iterate over each element (TrieNode) in the hashmap
        for (_, child_node) in hashmap.iter() {
            // recursively find the length of each child node
            let node_length: usize = (*child_node).get_num_children_and_self();

            // add node length to the count
            num_children += node_length;
        }
        
        return num_children;
    }

    // get iterator
    fn get_iter_node(&self) -> Vec<(char, Option<i32>)> {
        // empty vector
        let mut iter: Vec<(char, Option<i32>)> = Vec::new();

        // get the hashmap
        let hashmap = &self.chs;

        // iterate over each element (TrieNode) in the hashmap
        for (key, node) in hashmap.iter() {
            // add the direct children nodes to the vector
            iter.push((*key, node.value));

            // get a vector of iterators from their child nodes
            let mut child_node_iter: Vec<(char, Option<i32>)> = node.get_iter_node();

            // append the vectors
            iter.append(&mut child_node_iter);
        }

        return iter;
    }

    // iterate over child nodes of self, and return (char, Option())
    // where char is the first character of the key
    // where Option() is the node that is the head of the matching string
    fn find_node(&self, key: &String) -> (char, Option<&TrieNode>) {
        // split key into char, and rest of string
        let mut chars = key.chars(); // Create an iterator over chars
        let (first_char, rest_of_string) = chars.next().map(|first| (first, chars.collect::<String>())).unwrap_or((char::default(), String::new()));

        // get the hashmap
        let hashmap = &self.chs;

        // iterate over each element (TrieNode) in the hashmap
        for (child_node_key, child_node) in hashmap.iter() {
            // keys do not match (child node is irrelevant)
            if first_char != *child_node_key {
                continue;
            }

            match rest_of_string.as_str() {
                // no more string to match
                "" => {
                    match child_node.value {
                        // if value of node is not None, the key exists
                        Some(_) => return (first_char, Some(child_node)),
                        // key does not exist
                        None => return (first_char, None),
                    }
                },
                // string to match
                _ => {
                    // recurse into inner nodes
                    let track_valid: Option<&TrieNode>;
                    (_, track_valid) = child_node.find_node(&rest_of_string);

                    // if track_valid is None, inner nodes do not satisfy condition
                    match track_valid {
                        Some(_) => return (first_char, Some(child_node)),
                        None => return (first_char, None),
                        
                    }
                }
            }
        }

        // no nodes satisfy, return None
        return (first_char, None);
    }
}

impl Trie {
    fn new() -> Trie {
        Trie {
            root: TrieNode {
                chs: HashMap::new(),
                value: None,
            },
        }
    }

    fn add_string(&mut self, string: String, value: i32) {
        let mut current_node = &mut self.root;
        for c in string.chars() {
            // .chs accesses the hashmap
            // .entry(c) returns the node we are currently on (basically saying the letter exists already)
            // if.entry(c) returns nothing (we have not inserted this letter yet into this sequence)
            // we insert a new node as a child of the current_node using .or_insert()
            // and replace current_node wih the new child node
            current_node = current_node
                .chs
                .entry(c)
                .or_insert(TrieNode {
                    chs: HashMap::new(),
                    value: None,
                });
        }
        current_node.value = Some(value);
    }

    // get length of trie (number of key-value pairs)
    fn length(&self) -> usize {
        // get length (0 because root node value is None)
        let mut length: usize = 0;

        // get the root node
        let root_node: &TrieNode = &self.root;

        // get length of children
        length += root_node.get_num_children_and_self();

        return length;
        
    }

    // return iterator over key-value pairs
    fn iter(&self) -> Vec<(char, Option<i32>)> {
        // get root node
        let root_node: &TrieNode = &self.root;

        // get an iterator from the root node (does not include NONE value from root)
        let iter: Vec<(char, Option<i32>)> = root_node.get_iter_node();

        return iter;
    }

    // return a reference to the head of the TrieNode which contains the start of key (if it exists)
    fn find(&self, key: &String) -> Option<&TrieNode> {
        // get root node
        let root_node: &TrieNode = &self.root;
        
        // set exists to point to the root node
        let mut track_node: Option<&TrieNode> = None;

        (_, track_node) = root_node.find_node(key);

        return track_node;
    }

    // delete a node
    fn delete(&mut self, key: &String) {
        // get root node
        let root_node: &mut TrieNode = &mut self.root;

        root_node.delete_child_node_pathway(key);
    }

}

fn main() {
    let mut trie = Trie::new();
    trie.add_string("Bar".to_string(), 1);
    trie.add_string("Barey".to_string(), 2);

    // --- example of using length() ---
    // println!("Length of node is {}", trie.length());

    // --- example of using iter() ---
    // let trie_node_key_values: Vec<(char, Option<i32>)> = trie.iter();
    // for (key, value_option) in trie_node_key_values {
    //     print!("key: {} ", key);
    //     println!("value: {:?}", value_option);
    // }

    // --- example of using find() ---
    // let search_string: String = "Barry".to_string();
    // match trie.find(&search_string) {
    //     Some(node) => println!("Node found: {:?}", node),
    //     None => println!("Node does not exist"),
    // };
}


// --- unused code ---
    // for impl TrieNode
    // // same as find_node, but returns a mutable reference
    // fn find_node_mut(&mut self, key: &String) -> (char, Option<&mut TrieNode>) {
    //     // split key into char, and rest of string
    //     let mut chars = key.chars(); // Create an iterator over chars
    //     let (first_char, rest_of_string) = chars.next().map(|first| (first, chars.collect::<String>())).unwrap_or((char::default(), String::new()));
    
    //     // get the hashmap
    //     let hashmap = &mut self.chs;
    
    //     // iterate over each element (TrieNode) in the hashmap
    //     for (child_node_key, child_node) in hashmap.iter_mut() {
    //         // keys do not match (child node is irrelevant)
    //         if first_char != *child_node_key {
    //             continue;
    //         }
    
    //         match rest_of_string.as_str() {
    //             // no more string to match
    //             "" => {
    //                 match child_node.value {
    //                     // if value of node is not None, the key exists
    //                     Some(_) => return (first_char, Some(child_node)), // return mutable reference to child_node
    //                     // key does not exist
    //                     None => return (first_char, None),
    //                 }
    //             },
    //             // string to match
    //             _ => {
    //                 // recurse into inner nodes
    //                 let (_, track_valid) = child_node.find_node(&rest_of_string);
    
    //                 // if track_valid is None, inner nodes do not satisfy condition
    //                 match track_valid {
    //                     Some(_) => return (first_char, Some(child_node)), // return mutable reference to child_node
    //                     None => return (first_char, None),
    //                 }
    //             }
    //         }
    //     }
    
    //     // no nodes satisfy, return None
    //     return (first_char, None);
    // }
    

    // // // true if the node has children
    // // fn has_children(&self) -> bool {
    // //     return !self.chs.is_empty();
    // // }

    // // // remove the child node (identified by char) from the current TrieNode
    // // fn delete_child_node(&mut self, key: char) {
    // //     self.chs.remove(&key);
    // // }

    // // fn handle_deletion_child_node(&mut self, key: char) {
    // //     // Try to get the corresponding child node mutably
    // //     if let Some(node) = self.chs.get_mut(&key) {
    // //         // If the node has children, set its value to None to preserve the structure
    // //         if node.has_children() {
    // //             node.value = None;  // Disconnect the value while keeping children intact
    // //         }
    // //         // If the node has no children, just delete it
    // //         else {
    // //             self.delete_child_node(key);  // Remove the node from the HashMap
    // //         }
    // //     } else {
    // //         // The node with the given key doesn't exist
    // //         println!("Node with key '{}' not found", key);
    // //     }
    // // }

    // // // iterate over child nodes
    // // // if any nodes are fond to be the key, delete them
    // // fn delete_child_node_pathway(&mut self, key: &String) {
    // //     // get reference to child node that matches the key, and its char
    // //     let child_node_key: char;
    // //     let child_node_found: Option<&mut TrieNode>;
    // //     (child_node_key, child_node_found) = self.find_node_mut(key);

    // //     match child_node_found {
    // //         // reference is Some()
    // //         Some(child_node) => {
    // //             // if reference has children, iterate into children
    // //             if child_node.has_children() {
    // //                 // get rest of string
    // //                 let rest_of_string: String = key.chars().skip(1).collect::<String>();
    // //                 // recurse deletion into child node
    // //                 child_node.delete_child_node_pathway(&rest_of_string);
    // //             }
    // //             else {
    // //                 // specialized deletion of node
    // //                 self.handle_deletion_child_node(child_node_key);
    // //             }
    // //         },
    // //         // reference is None, return early
    // //         None => return,
    // //     };

    // //     return;
    // // }