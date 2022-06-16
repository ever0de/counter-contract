use crate::ContractError;

pub type Result<T, E = ContractError> = std::result::Result<T, E>;
