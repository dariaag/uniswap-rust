use fmt::std::{Debug, Display, Result as FmtResult};
use web3::types::{Address, U256};

//generic token
pub struct Token {
    address: Address,
    name: String,
    symbol: String,
    decimals: U256,
}

impl Token {
    pub fn new(address: Address, name: String, symbol: String, decimals: U256) -> Token {
        Token {
            address,
            name,
            symbol,
            decimals,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> FmtResult {
        write!(
            f,
            "Token Address: {:?} -- Name: {} -- Symbol: {} -- Decimals: {}",
            self.address, self.name, self.symbol, self.decimals
        )
    }
}
