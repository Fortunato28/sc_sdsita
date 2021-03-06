#![no_std]
#![allow(non_snake_case)]
#![feature(proc_macro_hygiene)]

extern crate pwasm_std;
extern crate pwasm_ethereum;
extern crate pwasm_abi;
extern crate pwasm_abi_derive;

pub mod token {
    use pwasm_ethereum;
    use pwasm_abi::types::*;

    // eth_abi is a procedural macros https://doc.rust-lang.org/book/first-edition/procedural-macros.html
    use pwasm_abi_derive::eth_abi;

    lazy_static::lazy_static! {
        static ref TOTAL_SUPPLY_KEY: H256 =
            H256::from(
                [2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
            );
        static ref OWNER_KEY: H256 =
            H256::from(
                [3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
            );
        static ref OWNER_ADDRESS: H160 =
            H160::from(
                [3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
            );
        static ref LINK_TO_TASK: H256 =
            H256::from(
                [3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
            );
        static ref MINIMUM_EXEC: H256 =
            H256::from(
                [3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
            );
        static ref MAXIMUM_EXEC: H256 =
            H256::from(
                [3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
            );
        static ref CURRENT_EXEC: H256 =
            H256::from(
                [3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
            );
        static ref AMOUNT_OF_ANSWERS: H256 =
            H256::from(
                [3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
            );
        static ref BLOCKS_ANOUNT_DEADLINE: H256 =
            H256::from(
                [3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
            );

        static ref ANSWERS: pwasm_std::Vec<Vec<u8>> = Vec::new();
        static ref TEST: [H256; 500] = [H256::zero(); 500];
    }

    #[eth_abi(TokenEndpoint, TokenClient)]
    pub trait TokenInterface {
        /// The constructor
        fn constructor(&mut self, _total_supply: U256, _task: H256, min_exec: H256, max_exec: H256, amount_of_block_deadline: H256);
        /// Total amount of tokens
        #[constant]
        fn get_task(&mut self) -> H256;
        #[constant]
        fn get_number_of_blocks_before_deadline(&mut self) -> H256;
        /// What is the reward for each executor?
        fn get_current_reward(&mut self) -> H256;
        /// Send result of calculations
        fn send_answer(&mut self, answer: U256) -> bool;
        /// Transfer the balance from owner's account to another account
        fn transfer_reward(&mut self, to: Address, _amount: U256) -> bool;
        /// Event declaration
        #[event]
        fn Transfer(&mut self, indexed_from: Address, indexed_to: Address, _value: U256);
    }

    pub struct TokenContract;

    impl TokenInterface for TokenContract {
        fn constructor(&mut self, total_supply: U256, _task: H256, min_exec: H256, max_exec: H256, nums_block_deadline: H256)
        {
            let sender = pwasm_ethereum::sender();
            // Set up the full reward about task
            pwasm_ethereum::write(&TOTAL_SUPPLY_KEY, &total_supply.into());
            // Give all money to the contract deployer
            pwasm_ethereum::write(&balance_key(&sender), &total_supply.into());
            // Set the contract owner
            pwasm_ethereum::write(&OWNER_KEY, &H256::from(sender).into());
            // Set link to task
            pwasm_ethereum::write(&LINK_TO_TASK, &H256::from(_task).into());
            // Set minimum amount of executors
            pwasm_ethereum::write(&MINIMUM_EXEC, &H256::from(min_exec).into());
            // Set maximum amount of executors
            pwasm_ethereum::write(&MAXIMUM_EXEC, &H256::from(max_exec).into());
            // Set current amount of executors
            pwasm_ethereum::write(&MAXIMUM_EXEC, &H256::zero().into());
            // Set maximum amount of executors
            pwasm_ethereum::write(&BLOCKS_ANOUNT_DEADLINE, &H256::from(nums_block_deadline).into());
        }

        fn get_task(&mut self) -> H256 {
            // Increment amount of executors
            let current_exec = U256::from_big_endian(&pwasm_ethereum::read(&CURRENT_EXEC)) + U256::one();
            pwasm_ethereum::write(&CURRENT_EXEC, &current_exec.into());
            pwasm_ethereum::read(&LINK_TO_TASK).into()
        }

        fn get_number_of_blocks_before_deadline(&mut self) -> H256 {
            pwasm_ethereum::read(&BLOCKS_ANOUNT_DEADLINE).into()
        }

        fn get_current_reward(&mut self) -> H256 {
            let contract_balance = U256::from_big_endian(&pwasm_ethereum::read(&TOTAL_SUPPLY_KEY));
            let current_exec = U256::from_big_endian(&pwasm_ethereum::read(&CURRENT_EXEC));
            (contract_balance / current_exec).into()
        }

        fn send_answer(&mut self, answer: U256) -> bool {
            let answers_amount = U256::from_big_endian(&pwasm_ethereum::read(&AMOUNT_OF_ANSWERS)) + U256::one();
            pwasm_ethereum::write(&AMOUNT_OF_ANSWERS, &answers_amount.into());
            pwasm_ethereum::write(&TEST[answers_amount.as_usize()], &answer.into());
            true
        }
        
        fn transfer_reward(&mut self, to: Address, amount: U256) -> bool {
            let owner_address = H160::from([3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
            // Execute only if contracts owner calls
            if pwasm_ethereum::sender() == owner_address {

                // For each executors
                let current_exec = U256::from_big_endian(&pwasm_ethereum::read(&CURRENT_EXEC)).as_usize();
                // TODO fix it with real iterations other the executors addresses
                for _executor in 1..current_exec {

                    // This is contract address itself
                    let contract_address = pwasm_ethereum::address();
                    let contract_balance = pwasm_ethereum::balance(&contract_address);
                    let recipientBalance = read_balance_of(&to);
                    
                    if amount == 0.into() || contract_balance < amount || to == contract_address {
                        return false;
                    } else {
                        let new_contract_balance = contract_balance - amount;
                        let new_recipient_balance = recipientBalance + amount;
                        pwasm_ethereum::write(&balance_key(&contract_address), &new_contract_balance.into());
                        pwasm_ethereum::write(&balance_key(&to), &new_recipient_balance.into());
                        self.Transfer(contract_address, to, amount);
                    }
                }
            true
            } else {
                false
            }
        }
    }

    // Reads balance by address
    fn read_balance_of(owner: &Address) -> U256 {
        U256::from_big_endian(&pwasm_ethereum::read(&balance_key(owner)))
    }

    // Generates a balance key for some address.
    // Used to map balances with their owners.
    fn balance_key(address: &Address) -> H256 {
        let mut key = H256::from(*address);
        key.as_bytes_mut()[0] = 1; // just a naive "namespace";
        key
    }
}
// Declares the dispatch and dispatch_ctor methods
use pwasm_abi::eth::EndpointInterface;

#[no_mangle]
pub fn call() {
    let mut endpoint = token::TokenEndpoint::new(token::TokenContract{});
    // Read http://solidity.readthedocs.io/en/develop/abi-spec.html#formal-specification-of-the-encoding for details
    pwasm_ethereum::ret(&endpoint.dispatch(&pwasm_ethereum::input()));
}

#[no_mangle]
pub fn deploy() {
    let mut endpoint = token::TokenEndpoint::new(token::TokenContract{});
    endpoint.dispatch_ctor(&pwasm_ethereum::input());
}

// TODO fix this tests, do it usefull
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    extern crate pwasm_test;
    extern crate std;
    use super::*;
    use core::str::FromStr;
    use pwasm_abi::types::*;
    use self::pwasm_test::{ext_reset, ext_get};
    use token::TokenInterface;

    #[test]
    fn should_succeed_transfering_1000_from_owner_to_another_address() {
        let mut contract = token::TokenContract{};
        let owner_address = Address::from_str("ea674fdde714fd979de3edf0f56aa9716b898ec8").unwrap();
        let sam_address = Address::from_str("db6fd484cfa46eeeb73c71edee823e4812f9e2e1").unwrap();
        // Here we're creating an External context using ExternalBuilder and set the `sender` to the `owner_address`
        // so `pwasm_ethereum::sender()` in TokenInterface::constructor() will return that `owner_address`
        ext_reset(|e| e.sender(owner_address.clone()));
        let total_supply = 10000.into();
        contract.constructor(total_supply);
        assert_eq!(contract.balanceOf(owner_address), total_supply);
        assert_eq!(contract.transfer(sam_address, 1000.into()), true);
        assert_eq!(contract.balanceOf(owner_address), 9000.into());
        assert_eq!(contract.balanceOf(sam_address), 1000.into());
        // 1 log entry should be created
        assert_eq!(ext_get().logs().len(), 1);
    }

    #[test]
    fn should_not_transfer_to_self() {
        let mut contract = token::TokenContract{};
        let owner_address = Address::from_str("ea674fdde714fd979de3edf0f56aa9716b898ec8").unwrap();
        ext_reset(|e| e.sender(owner_address.clone()));
        let total_supply = 10000.into();
        contract.constructor(total_supply);
        assert_eq!(contract.balanceOf(owner_address), total_supply);
        assert_eq!(contract.transfer(owner_address, 1000.into()), false);
        assert_eq!(contract.balanceOf(owner_address), 10000.into());
        assert_eq!(ext_get().logs().len(), 0);
    }

}


// Что вообще нужно чувакам, использующим контракт? А вот что:
//1. Автор деплоит контракт;
//2. Указывает исходные данные (где взять задание, сумма оплаты, количество денег на контракте, максимальное количество исполнителей, когда дедлайн);
//3. Исполнитель должен взять задание;
//4. Исполнитель должен вернуть результат задания и получить деньги;
