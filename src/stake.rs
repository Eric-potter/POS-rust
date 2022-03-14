use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stake {
    pub accounts: Vec<String>,
    pub balances: HashMap<String, u64>,
}

impl Stake {
    pub fn new() -> Self {
        Self {
            accounts: vec![String::from(
                "5aad9b5e21f63955e8840e8b954926c60e0e2d906fdbc0ce1e3afe249a67f614",
            )],
            balances: HashMap::from([(
                String::from("5aad9b5e21f63955e8840e8b954926c60e0e2d906fdbc0ce1e3afe249a67f614"),
                10,
            )]),
        }
    }

    pub fn initialize(&mut self, address: &String) {
        if !self.balances.contains_key(address) {
            self.balances.insert(address.to_string(), 0);
            self.accounts.push(address.to_string());
        }
    }

    pub fn add_stake(&mut self, from: &String, amount: &u64) {
        self.initialize(from);
        *self.balances.get_mut(from).unwrap() += amount;
    }

    pub fn get_max(&mut self, addresses: &Vec<String>) -> String {
        let key = self
            .balances
            .iter()
            .filter(|addr| addresses.contains(&addr.0))
            .collect::<HashMap<_, _>>();
        key.iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
            .unwrap()
            .to_string()
    }

    pub fn update(&mut self, txn: &Transaction) {
        self.add_stake(&txn.txn_input.from, &(*&txn.txn_output.amount as u64))
    }
}