use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress {
  fn convert_address(&self) -> Result<Address, &'static str>;
}

impl EthereumAddress for &str {
  fn convert_address(&self) -> Result<Address, &'static str> {
    match Address::from_str(self) {
      Ok(address) => Ok(address),
      Err(_) => Err("Invalid Ethereum Address String")
    }
  }
}

impl EthereumAddress for Address {
  fn convert_address(&self) -> Result<Address, &'static str> {
      Ok(*self)
  }
}

#[allow(dead_code)]
fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
  address.convert_address().unwrap()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_polymorfism() {
    let address_str = Address::from_str("0xdAC17F958D2ee523a2206206994597C13D831ec7").unwrap();
    let address = get_ethereum_data(address_str);

    assert_eq!(address, Address::from_str("0xdAC17F958D2ee523a2206206994597C13D831ec7").unwrap());

    let new_addr = get_ethereum_data("0xdAC17F958D2ee523a2206206994597C13D831ec7");
    assert_eq!(new_addr, Address::from_str("0xdAC17F958D2ee523a2206206994597C13D831ec7").unwrap());

    assert_eq!(address_str, address);
  }
}