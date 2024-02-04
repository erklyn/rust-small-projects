trait Account {
    fn deposit(&mut self, amount: u32);
    fn withdraw(&mut self, amount: u32);
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
    fn withdraw(&mut self, amount: u32) {
        if self.balance < amount as i32 {
            println!(
                "trying to withdraw {}, but balance is {}",
                amount, self.balance
            )
        }
        self.balance -= amount as i32
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

    account_b.deposit(2000);
    account_b.withdraw(100);
    println!(
        "Balance: {} Holder: {}  A.N.: {}",
        account_b.balance(),
        account_b.holder_name,
        account_b.account_number
    );
    account_a.deposit(1512512);
    account_a.withdraw(112512);
    println!(
        "Balance: {} Holder: {}  A.N.: {}",
        account_a.balance(),
        account_a.holder_name,
        account_a.account_number
    );
}
