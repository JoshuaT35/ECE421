use std::{
    sync::{Arc, Mutex},
    thread,
};


#[derive(Clone,Debug)]
struct Bank {
    accounts: Arc<Mutex<Vec<i32>>>,
}
impl Bank {
    fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(0);
        }
        Bank {
            accounts: Arc::new(Mutex::new(v)),
        }
    }

    // NOTE: assumes that accounts have enough money. What if they don't? For now, set the account value to 0
    fn transfer(&self, from: usize, to: usize, amount: i32) -> Result<(),()> {
        // get the larger value btwn from and to
        let mut larger: usize = from;
        if to > from {
            larger = to;
        }

        // if vector length does not have index of larger, an account is not contained in it
        {
            let mut accounts_locked = self.accounts.lock().unwrap();
            if larger > accounts_locked.len()-1 {
                return Err(());
            }

            // subtract/add to accounts
            if accounts_locked[from] < amount {
                accounts_locked[from] = 0;
            }
            else {
                accounts_locked[from] -= amount;
            }
            
            accounts_locked[to] += amount;
        }

        println!("Amount of ${} transferred from account id: {} to account id: {}", amount, from, to);

        return Ok(());
    }
}

struct Person {
    ac_id: usize,
    buddy_id: usize,
}

impl Person {
    pub fn new(id: usize, b_id: usize) -> Self {
        Person {
            ac_id: id,
            buddy_id: b_id,
        }
    }
}

fn main() {
    // num accounts
    let num_accounts = 10;

    // create a bank
    let mut bank: Arc<Bank> = Arc::new(Bank::new(num_accounts));

    // store thread handles
    let mut main_thread_handles = Vec::with_capacity(num_accounts);

    for i in 0..num_accounts {
        // create a Person
        let mut person: Person;
        if i != num_accounts-1 {
            person = Person::new(i, i+1);
        }
        else {
            person = Person::new(i, 0);
        }

        // clone a pointer to Bank to pass into thread
        let bank_ptr = Arc::clone(&bank);

        // spawn a thread
        let thread_handle = thread::spawn(move || {
            // amount to transfer
            let amount: i32 = 100;

            // move person and bank into the thread

            // transfer money
            bank_ptr.transfer(person.ac_id, person.buddy_id, amount).unwrap();
        });

        // store handle
        main_thread_handles.push(thread_handle);
    }

    // main waits
    for handle in main_thread_handles {
        handle.join().unwrap();
    }
}
