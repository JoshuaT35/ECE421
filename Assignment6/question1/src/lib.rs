use bcrypt::{
    DEFAULT_COST,
    hash,
    verify,
    BcryptError,
};
use sqlite::{
    Error as SqErr,
    State,
};
use chrono::{NaiveDateTime, Timelike};

pub struct UserBase {
    fname: String,
}

#[derive(Debug)]
pub enum UBaseErr {
    DbErr(SqErr),
    HashError(BcryptError),
}

// --- error implementations ---
impl From<SqErr> for UBaseErr {
    fn from(s: SqErr) -> Self {
        UBaseErr::DbErr(s)
    }
}

impl From<BcryptError> for UBaseErr {
    fn from(b: BcryptError) -> Self {
        UBaseErr::HashError(b)
    }
}

// --- struct functions ---
impl UserBase {
    // construct new UserBase since fields are private
    pub fn new(fname: String) -> Self {
        Self {
            fname: fname,
        }
    }
    // to get the fname since it is private
    pub fn get_fname(&self) -> &str {
        &self.fname
    }

    // add new user
    pub fn add_user(&self, u_name: &str, p_word: &str) -> Result<(), UBaseErr> {
        // open connection to the database
        let conn = sqlite::open(&self.fname)?;
        conn.execute("PRAGMA foreign_keys = ON;")?;

        // insert user data into users table
        let hpass = bcrypt::hash(p_word, DEFAULT_COST)?;
        let mut st = conn.prepare(
            "INSERT INTO users(u_name, p_word) VALUES (?,?);"
        )?;
        st.bind((1, u_name))?;
        st.bind((2, &hpass as &str))?;
        st.next()?;

        // initialize user balance into balances table to 1000
        st = conn.prepare(
            "INSERT INTO balances(u_name, balance) VALUES (:user, 1000);"
        )?;
        st.bind((":user", u_name))?;
        st.next()?;

        //
        Ok(())
    }

    // set a user's balance
    pub fn set_user_balance(&self, u_name: &str, balance: i64) -> Result<(), UBaseErr> {
        // open connection to the database
        let conn = sqlite::open(&self.fname)?;
        conn.execute("PRAGMA foreign_keys = ON;")?;

        // insert or update user balance in balances table
        // excluded.balance is the new balance we want to insert
        let mut st = conn.prepare(
            "INSERT INTO balances (u_name, balance) 
            VALUES (:user, :balance)
            ON CONFLICT(u_name) DO UPDATE SET balance = excluded.balance;"
        )?;
        st.bind((":user", u_name))?;
        st.bind((":balance", balance))?;
        st.next()?;

        //
        Ok(())
    }

    // get a user's balance
    pub fn get_user_balance(&self, u_name: &str) -> Result<i64, UBaseErr> {
        // open connection to the database
        let conn = sqlite::open(&self.fname)?;
        conn.execute("PRAGMA foreign_keys = ON;")?;

        // user balance in balances table
        let mut st = conn.prepare(
            "SELECT balance
            FROM balances
            WHERE u_name = :user"
        )?;
        st.bind((":user", u_name))?;

        // read and return the balance
        let mut balance_int: i64 = 0;
        if let Ok(State::Row) = st.next() {
            let balance: String = st.read::<String, _>("balance")?;
            balance_int = balance.parse::<i64>().unwrap();
        }

        Ok(balance_int)
    }

    // add new payment
    pub fn pay(&self, u_from: &str, u_to: &str, amount: i64) -> Result<(), UBaseErr> {
        // get balance of users
        let user_from_balance: i64 = self.get_user_balance(u_from)?;
        let user_to_balance: i64 = self.get_user_balance(u_to)?;

        // if balance of sending user is <= amount (not enough money) then panic
        if user_from_balance <= amount {
            panic!("pay: user does not have enough money");
        }

        // open database
        let conn = sqlite::open(&self.fname)?;
        conn.execute("PRAGMA foreign_keys = ON;")?;

        // insert transaction into transactions
        let mut st = conn.prepare(
            "INSERT INTO transactions(u_from, u_to, t_date, t_amount)
            VALUES (?,?,datetime(\"now\"),?);"
        )?;
        st.bind((1, u_from))?;
        st.bind((2, u_to))?;
        st.bind((3, amount))?;
        st.next()?;

        // insert new balance into balances
        self.set_user_balance(u_from, user_from_balance-amount)?;
        self.set_user_balance(u_to, user_to_balance+amount)?;

        Ok(())
    }

    // return transaction history
    pub fn get_transactions_history(&self, u_name: &str) ->Result<(),UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        conn.execute("PRAGMA foreign_keys = ON;")?;

        let mut st = conn.prepare(
            "SELECT *
            FROM transactions
            WHERE u_from = ? OR u_to = ?"
        )?;
        // bind actual parameters to question marks
        st.bind((1, u_name))?;
        st.bind((2, u_name))?;
        
        // go through each statement and print
        while let Ok(State::Row) = st.next() {
            // read values
            let u_from: String = st.read::<String, _>("u_from")?;
            let u_to: String = st.read::<String, _>("u_to")?;
            let t_date: String = st.read::<String, _>("t_date")?;
            let t_amount: String = st.read::<String, _>("t_amount")?;

            // Parse t_date as NaiveDateTime (assuming the format is "YYYY-MM-DD HH:MM:SS")
            let naive_datetime = NaiveDateTime::parse_from_str(&t_date, "%Y-%m-%d %H:%M:%S")
            .expect("Failed to parse date");

            // format date
            let formatted_datetime = naive_datetime.format("%m/%d/%Y at %I:%M %p").to_string();

            // if u_from is the user, print this statement
            if u_from == u_name {
                println!("{} sent ${} to {} on {}", u_from, t_amount, u_to, formatted_datetime);
            }
            // else, print this other statement
            else if u_to == u_name {
                println!("{} received ${} from {} on {}", u_from, t_amount, u_to, formatted_datetime);
            }
            else {
                panic!("get_transactions_history: selected row does not feature {}", u_name);
            }
        }

        Ok(())
    }
}


// --- tests ---