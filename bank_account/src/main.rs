struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }
    fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            println!("You dont have enough balance");
        } else {
            self.balance -= amount;
        }
    }
    fn display(&self) {
        println!("{} has {}", self.owner, self.balance);
    }
}

fn main() {
    let mut account = BankAccount {
        owner: String::from("Tanzim Rizwan"),
        balance: 27007.15,
    };

    account.deposit(50.24);
    account.withdraw(11.50);
    account.display();
}
