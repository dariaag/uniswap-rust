use crate::token::Token;
use web3::contract::Contract;
use web3::transports::Http;
use web3::types::{Address, U256};
use web3::Web3;

pub struct Uniswap {
    web3: Web3<Http>,
    factory: Contract<Address>,
    quoter: Contract<Address>,
}

impl Uniswap {
    fn get_token_input_price(
        &self,
        &token_1: Address,
        &token_2: Address,
        &qty: u256,
        &fee: u256,
    ) -> u256 {
        let sqrt_price_limit = 0;
        let data = self
            .quoter
            .call(
                "quoteInputSingle",
                (token_1, token_2, qty, sqrt_price_limit, fee),
                None,
            )
            .unwrap();
        return U256::from_big_endian(&data);
    }

    fn get_token_output_price(
        &self,
        &token_1: Address,
        &token_2: Address,
        &qty: u256,
        &mut fee: u256,
    ) -> u256 {
        if fee == 0 {
            println!("assuming fee is 0.3%");
            fee = 3000;
        }
        sqrt_price_limit = 0;
        let data = self
            .quoter
            .call(
                "quoteExactOutputSingle",
                (token_1, token_2, qty, sqrt_price_limit, fee),
                None,
            )
            .unwrap();
        return U256::from_big_endian(&data);
    }
    fn create_transaction<P, D>(&self, &contract: Contract, &func: String, &tx_params: P) -> D {
        let data = contract.call(func, tx_params, None).unwrap();
        return data;
    }
}
