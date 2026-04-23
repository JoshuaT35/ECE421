extern crate nom;
extern crate term_size;

use std::io;

use nom::{
    branch::alt,
    IResult,
    bytes::complete::tag,
    Parser,
};
use red_black_tree::tree::BaseTree;
use red_black_tree::tree::Tree;
use red_black_tree::rbtree::RBTree;
use ptree::print_tree;

#[derive(Debug)]
enum Cmd<'a> {
    Add(isize),
    Delete(isize),
    Print,
    Quit,
    Clear,
    Help(&'a str),
    New(TreeSelection),
    Count,
    Height,
    Commands,
    CheckEmpty,
    InOrder
}

fn command(input: &str) -> IResult<&str, Cmd> {
    // CURRENTLY CASE SENSITIVE, MUST ALL BE LOWERCASE
    let Ok((parsed_args, parsed_cmd)) = alt((tag::<_, _, nom::error::Error<&str>>("quit"), 
                                             tag::<_, _, nom::error::Error<&str>>("print"), 
                                             tag::<_, _, nom::error::Error<&str>>("clear"), 
                                             tag::<_, _, nom::error::Error<&str>>("help"), 
                                             tag::<_, _, nom::error::Error<&str>>("add"), 
                                             tag::<_, _, nom::error::Error<&str>>("delete"), 
                                             tag::<_, _, nom::error::Error<&str>>("count"), 
                                             tag::<_, _, nom::error::Error<&str>>("height"),
                                             tag::<_, _, nom::error::Error<&str>>("in-order"),
                                             tag::<_, _, nom::error::Error<&str>>("commands"),                                             
                                             tag::<_, _, nom::error::Error<&str>>("check-empty"),                                             
                                             tag::<_, _, nom::error::Error<&str>>("new")))
                                             .parse(input)
                                             else {
                                                return Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
                                             }; 
    match parsed_cmd {
        "clear" => Ok((parsed_args, Cmd::Clear)),
        "quit" => Ok((parsed_args, Cmd::Quit)),
        "print" => Ok((parsed_args, Cmd::Print)),
        "check-empty" => Ok((parsed_args, Cmd::CheckEmpty)),
        "help" => {
            let (_input, help_str) = nom::character::complete::alpha1(parsed_args.trim())?;
            Ok((parsed_args, Cmd::Help(help_str)))
        }
        "add" => {
            let (_input, num_str) = nom::character::complete::digit1(parsed_args.trim())?;
            let num = num_str.parse::<isize>().unwrap();
            Ok((parsed_args, Cmd::Add(num)))
        },
        "delete" => {
            let (_input, num_str) = nom::character::complete::digit1(parsed_args.trim())?;
            let num = num_str.parse::<isize>().unwrap();
            Ok((parsed_args, Cmd::Delete(num)))
        },
        "new" => {
            let Ok((_input, tree_type)) = alt((tag::<_, _, nom::error::Error<&str>>("rb"), 
                                              tag::<_, _, nom::error::Error<&str>>("avl")))
                                              .parse(parsed_args.trim())
                                              else {
                                                return Err(nom::Err::Failure(nom::error::Error::new(parsed_args, nom::error::ErrorKind::Tag)));
                                             };
            let cmd = match tree_type {
                "rb" => Cmd::New(TreeSelection::RedBlack),
                _ => return Err(nom::Err::Failure(nom::error::Error::new(parsed_args, nom::error::ErrorKind::Tag))),
            };
            Ok((parsed_args, cmd))
        },
        "count" => Ok((parsed_args, Cmd::Count)),
        "height" => Ok((parsed_args, Cmd::Height)),
        "commands" => Ok((parsed_args, Cmd::Commands)),
        "in-order" => Ok((parsed_args, Cmd::InOrder)),

        _ => Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Tag))),
    }
}


fn eval(
    cmd: Cmd,
    rb: &mut RBTree<isize>,
    tree_type: &mut TreeSelection,
) {
    match cmd {
        Cmd::Help(v) => match v {
            "quit" => println!("Exits the program\n"),
            "print" => println!("Displays the current tree\n"),
            "add" => println!("Inserts a node with a value specified in the format: add [value], where value \
                               is a non-negative integer. \nAny non-integer values will be rounded\n"),
            "delete" => println!("Deletes a node with a value specified in the format: delete [value], where value \
                               is a non-negative integer. \nAny non-integer values will be rounded\n"),
            "clear" => println!("Removes all nodes from the current tree\n"),
            "new" => println!("Creates a new tree specified in the format: new [tree], where tree is either AVL or RB\n"),
            "count" => println!("Displays the number of leaf nodes in the current tree\n"),
            "height" => println!("Displays the height of the current tree\n"),
            "commands" => println!("Displays all the available commands\n"),
            "help" => println!("Provides a description of the specified function\n"),
            "check-empty" => println!("Checks if a created tree is empty"),
            "in-order" => println!("Displays the in-order traversal of the current tree\n"),
            &_ => eprintln!("Not a valid command to ask for help\n"),
        }
        Cmd::Quit => {
            std::process::exit(0);
        }
        Cmd::Clear => {
            *rb = RBTree::new();
            println!("Tree has been cleared\n");
        }
        Cmd::Print => match tree_type {
            TreeSelection::RedBlack => {
                if let Some(root) = rb.get_root() {
                    print_tree(rb).expect("Failed to print tree");
                    println!("");
                } else {
                    println!("Empty Red Black Tree\n")
                }
            }
            TreeSelection::Undefined => eprintln!("No tree created\n"),
        },
        Cmd::CheckEmpty => {
            if tree_type == &TreeSelection::Undefined {
                println!("No tree created\n");
            } else if rb.is_empty() {
                println!("Tree is empty\n");
            } else {
                println!("Tree is not empty\n");
            }
        }
        Cmd::InOrder =>  {
            if tree_type == &TreeSelection::Undefined {
                println!("No tree created\n");
            } else if rb.is_empty() {
                println!("Tree is empty\n");
            } else {
                rb.inorder_traversal();
            }
        }
        Cmd::Add(v) => match tree_type {
            TreeSelection::RedBlack => {
                rb.insert(v);
                println!("{} added to the tree\n", v);
            },
            TreeSelection::Undefined => eprintln!("No tree created\n"),
        },
        Cmd::Delete(v) => match tree_type {
            TreeSelection::RedBlack => {
                if rb.delete(v) {
                    println!("{} deleted from the tree\n", v);
                } else {
                    println!("A node with value {} could not be found in the tree\n", v);
                }
            }
            TreeSelection::Undefined => eprintln!("No tree created\n"),
        },
        Cmd::Commands => {
            println!("Commands:");
            println!("  new [AVL | RB]");
            println!("  add [val]");
            println!("  delete [val]");
            println!("  print");
            println!("  clear");
            println!("  quit");
            println!("  commands");
            println!("  help\n");
        }
        Cmd::New(v) => {
            *tree_type = v;
            *rb = RBTree::new();
            println!("New red-black tree created\n");
        }
        Cmd::Count => match tree_type {
            TreeSelection::RedBlack => {
                let lc = rb.get_leaf_count();
                println!("Number of leaves in the tree: {}\n", lc);
            }
            TreeSelection::Undefined => eprintln!("No tree created\n"),
        },
        Cmd::Height => match tree_type {
            TreeSelection::RedBlack => {
                let tree_height = rb.get_height();
                println!("Height of the tree: {}\n", tree_height);
            }
            TreeSelection::Undefined => eprintln!("No tree created\n"),
        }
    }
}

fn read_and_eval(
    mut rb: &mut RBTree<isize>,
    tree_type: &mut TreeSelection,
) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input_cmd: String = input.trim().parse::<String>().expect("Invalid command. Try: help [command]"); 
    let ss = input_cmd.as_str().to_string();
    let result = command(&ss);
    if let Ok((_s, cmd)) = result {
        eval(cmd, &mut rb, tree_type);
    } else {
        println!("Invalid command. Try: help [command]\n");
    }
}

#[derive(Debug, PartialEq)]
enum TreeSelection {
    RedBlack,
    Undefined,
}
fn main() {
    std::process::Command::new("clear").status().unwrap();
    println!("ECE 421 Project 2");
    println!("Enter one of the following commands:");
    println!("  new [AVL | RB]");
    println!("  add [val]");
    println!("  delete [val]");
    println!("  count");
    println!("  height");
    println!("  print");
    println!("  check-empty");
    println!("  in-order");
    println!("  clear");
    println!("  quit");
    println!("  commands");
    println!("  help");
    println!("To view the list of commands again, use: commands");
    println!("To view the description of a command, use: help [command]\n");
    let mut rbtree = RBTree::new();
    let mut tree_type = TreeSelection::Undefined;
    loop {
        read_and_eval(&mut rbtree, &mut tree_type);
    };
}