use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub accounts: Vec<String>,
    pub balances: HashMap<String, f64>,
}

impl Account {
    pub fn new() -> Self {
        Self {
            accounts: vec![String::from(
                "5aad9b5e21f63955e8840e8b954926c60e0e2d906fdbc0ce1e3afe249a67f614",
            )],
            balances: HashMap::from([(
                String::from("5aad9b5e21f63955e8840e8b954926c60e0e2d906fdbc0ce1e3afe249a67f614"),
                1000.00,
            )]),
        }
    }

    pub fn initialize(&mut self, address: &String) {
        if !self.balances.contains_key(address) {
            self.balances.insert(address.to_string(), 0.00);
            self.accounts.push(address.to_string());
        }
    }

    pub fn transfer(&mut self, from: &String, to: &String, amount: f64) {
        self.initialize(from);
        self.initialize(to);
        self.increment(to, amount);
        self.decrement(from, amount);
    }

    fn increment(&mut self, to: &String, amount: f64) {
        (*self.balances.get_mut(to).unwrap()) += amount;
    }

    fn decrement(&mut self, from: &String, amount: f64) {
        (*self.balances.get_mut(from).unwrap()) -= amount;
    }

    fn getBalance(&mut self, address: &String) -> &f64 {
        self.initialize(address);
        self.balances.get(address).unwrap()
    }
}