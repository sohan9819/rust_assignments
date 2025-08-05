use std::mem;

#[derive(Debug)]
struct Cash {
    total_amount: u32,
}

impl Cash {
    fn new(val: u32) -> Box<Self> {
        let cash = Box::new(Cash { total_amount: val });
        println!(
            "--------------\n Cash Created \n Stack Address : {:p} \n Heap Address : {:p}\n--------------",
            &cash, cash
        );
        println!(
            "--------------\n Cash \n Heap Address : {:p} \n--------------",
            &cash.total_amount
        );
        println!(
            "--------------\n Size Info : {} bytes \n--------------",
            mem::size_of::<Box<Cash>>()
        );
        cash
    }

    fn withdraw(&mut self, amount: u32) {
        println!("--------------");
        if self.total_amount >= amount {
            self.total_amount -= amount;
            println!("Withdrawed Amount : {}", amount);
            println!("Amount in ATM : {}", self.total_amount);
        } else {
            println!("Withdrawed Amount : {}", self.total_amount);
            println!("Withdrawed Failed Amount : {}", amount - self.total_amount);
            self.total_amount = 0;
            println!("Amount in ATM : {}", self.total_amount);
        }
        println!("--------------")
    }
}

impl Drop for Cash {
    fn drop(&mut self) {
        println!("ATM machine closed");
    }
}

fn main() {
    let mut atm = Cash::new(500);

    println!("ATM : {:#?}", &atm);

    atm.withdraw(100);
    atm.withdraw(150);
    atm.withdraw(300);
    atm.withdraw(100);

    println!("ATM : {:#?}", &atm);
}
