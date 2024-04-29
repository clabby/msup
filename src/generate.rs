//! This module contains CLI prompts for generating a [MultisigBatch] definition from user input, with type safety.

use crate::types::{BatchTransaction, MultisigBatch, ObjectMetadata};
use crate::util::encode_function_args;
use alloy_json_abi::Function;
use alloy_primitives::{hex::FromHex, Address, U256};
use anyhow::Result;
use inquire::Text;
use std::path::PathBuf;
use std::{fs::File, io::Write};
use yansi::Paint;

pub(crate) fn generate_batch_definition(output_path: &PathBuf) -> Result<()> {
    let num_transactions = u64::from_str_radix(
        &Text::new("Number of transactions in the multisig batch:").prompt()?,
        10,
    )?;

    let mut batch_definition = MultisigBatch {
        chain_id: u64::from_str_radix(
            &Text::new("Chain ID that the batch transaction will be performed on:").prompt()?,
            10,
        )?,
        metadata: ObjectMetadata {
            name: Text::new("Enter the name of the batch:").prompt()?,
            description: Text::new("Enter the description of the batch:").prompt()?,
        },
        transactions: Vec::default(),
    };

    // Fill the batch transactions
    (0..num_transactions).try_for_each(|i| {
        let tx = prompt_batch_transaction(i + 1)?;
        batch_definition.transactions.push(tx);
        Ok::<_, anyhow::Error>(())
    })?;

    let mut output = File::create("input.json")?;
    output.write_all(serde_json::to_string_pretty(&batch_definition)?.as_bytes())?;

    println!("Batch definition saved to {}", output_path.display().cyan());
    Ok(())
}

/// Prompts the user to specify a [BatchTransaction].
fn prompt_batch_transaction(i: u64) -> Result<BatchTransaction> {
    println!("{}", format!("Transaction #{}", i).bold().green());

    let tx_metadata = ObjectMetadata {
        name: Text::new("Name:").prompt()?,
        description: Text::new("Description:").prompt()?,
    };

    let to = Address::from_hex(&Text::new("Address of the contract to call:").prompt()?)?;
    let value = U256::from_str_radix(&Text::new("Value to send (in WEI):").prompt()?, 10)?;

    let contract_signature = Text::new("Enter the function signature of the contract to call")
        .with_help_message("Example: `deposit(uint256 amount)(bytes32 depositHash)`")
        .prompt()?;
    let mut function = Function::parse(&contract_signature)?;
    function
        .inputs
        .iter_mut()
        .enumerate()
        .for_each(|(i, input)| {
            input.name = (input.name.is_empty())
                .then(|| format!("unnamed_param{}", i))
                .unwrap_or_else(|| input.name.clone())
        });

    let inputs = prompt_raw_function_inputs(&function)?;
    let input_map = function
        .inputs
        .iter()
        .enumerate()
        .map(|(i, input)| (input.name.clone(), inputs[i].clone()))
        .collect();

    Ok(BatchTransaction {
        metadata: tx_metadata,
        to,
        value,
        data: encode_function_args(&function, inputs)?.into(),
        contract_method: function,
        contract_inputs_values: input_map,
    })
}

/// Prompts the user to specify the inputs for a function.
fn prompt_raw_function_inputs(function: &Function) -> Result<Vec<String>> {
    function
        .inputs
        .iter()
        .enumerate()
        .map(|(i, input)| {
            let input_value = Text::new(&format!(
                "Enter the value for input #{} ({}):",
                i + 1,
                input.name
            ))
            .prompt()?;

            Ok(input_value)
        })
        .collect()
}
