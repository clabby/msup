//! Utility functions for `msup`

use alloy_dyn_abi::{DynSolType, DynSolValue, JsonAbiExt};
use alloy_json_abi::Function;
use anyhow::Result;

/// Given a function and a vector of string arguments, it proceeds to convert the args to alloy
/// [DynSolValue]s and then ABI encode them.
pub fn encode_function_args<I, S>(func: &Function, args: I) -> Result<Vec<u8>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let params = std::iter::zip(&func.inputs, args)
        .map(|(input, arg)| coerce_value(&input.selector_type(), arg.as_ref()))
        .collect::<Result<Vec<_>>>()?;
    func.abi_encode_input(params.as_slice()).map_err(Into::into)
}

/// Helper function to coerce a value to a [DynSolValue] given a type string
pub fn coerce_value(ty: &str, arg: &str) -> Result<DynSolValue> {
    let ty = DynSolType::parse(ty)?;
    Ok(DynSolType::coerce_str(&ty, arg)?)
}
