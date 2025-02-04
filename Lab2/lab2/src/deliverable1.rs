fn main(){ 
    let mut groups = [[""; 4]; 6]; 
    groups[0]=["Bob", "Carol", "Eric", "Matt"]; 
    groups[1]=["Jim", "Lucy", "Terry", "Brenda"]; 
    groups[2]=["Susan", "Brad", "Jim", "Matt"]; 
    groups[3]=["Sue", "Wendy", "Sam", "Brad"]; 
    groups[4]=["Kate", "Jack", "James", "Sydney"]; 
    groups[5]=["Mary", "John", "Ricky", "Wendy"];

    searchMember(groups, "Bob");
}

fn searchMember(groups: [[&str; 4]; 6], name: &str) {
    let mut member_of_grps = Vec::new();
    let mut leader_of_grps = Vec::new();

    // iterate through groups
    for (_, group_itr) in groups.iter().enumerate() {
        for (j, name_itr) in group_itr.iter().enumerate() {
            // if member appears in group, add it to vector of groups it is in
            if *name_itr == name {
                member_of_grps.push(j);
                // if member is first name in group, add it to vector of group it is the leader of
                if j == 0 {
                    leader_of_grps.push(j);
                }
            }
        }
    }

    // print results
    if member_of_grps.len() > 0 {
        print!("{:?} is a member of groups: ", name);
        for grp_idx in &member_of_grps {
            print!("{:?} ", grp_idx+1);
        }
        println!();
        print!("{:?} is a leader of groups: ", name);
        for grp_idx in &leader_of_grps {
            print!("{:?} ", grp_idx+1);
        }
    }
    else {
        print!("{:?} is not a member of any group.", name);
    }
    println!();
}