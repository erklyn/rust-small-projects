trait Account {
    fn deposit(&mut self, amount: u32);
    fn withdraw(&mut self, amount: &u32) -> Result<&str, &str>;
    fn balance(&self) -> i32;
}

struct BankAccount {
    account_number: i32,
    balance: i32,
    holder_name: String,
}
impl Account for BankAccount {
    fn deposit(&mut self, amount: u32) {
        self.balance += amount as i32;
    }
    fn balance(&self) -> i32 {
        self.balance
    }
    fn withdraw(&mut self, amount: &u32) -> Result<&str, &str> {
        if self.balance < *amount as i32 {
            return Err("Cannot withdraw more than balance, check balance and try again.");
        }
        self.balance -= *amount as i32;
        Ok("Succesfully withdrawed the amount")
    }
}

fn main() {
    let mut account_a = BankAccount {
        holder_name: "Veli Uysal".to_string(),
        balance: 124124124,
        account_number: 1,
    };
    let mut account_b = BankAccount {
        account_number: 124125125,
        balance: 2000,
        holder_name: "Utku Enes GURSEL".to_string(),
    };

    let withdraw_amount = 100;
    account_b.deposit(2000);
    match account_b.withdraw(&withdraw_amount) {
        Ok(msg) => println!("{}", msg),
        Err(msg) => println!("{}", msg),
    }
    println!(
        "Balance: {} Holder: {}  A.N.: {}",
        account_b.balance(),
        account_b.holder_name,
        account_b.account_number
    );
    account_a.deposit(1512512);
    match account_a.withdraw(&withdraw_amount) {
        Ok(msg) => println!("{}", msg),
        Err(msg) => println!("{}", msg),
    }
    println!(
        "Balance: {} Holder: {}  A.N.: {}",
        account_a.balance(),
        account_a.holder_name,
        account_a.account_number
    );
}
