//! This file contains functions to generate a LaTeX document from a [MultisigBatch].

use crate::{
    types::{BatchTransaction, MultisigBatch},
    util::encode_function_args,
};
use alloy_primitives::Bytes;
use anyhow::{anyhow, Result};
use std::{fs::File, io::Write, path::PathBuf};
use yansi::Paint;

/// Renders a Markdown document from a [MultisigBatch] definition.
pub fn render_batch_doc(input: &PathBuf, output: &PathBuf) -> Result<()> {
    let contents = std::fs::read_to_string(input)?;
    let multisig_batch: MultisigBatch = serde_json::from_str(&contents)?;

    let mut document = String::new();
    append_header(&mut document, &multisig_batch);
    multisig_batch
        .transactions
        .iter()
        .enumerate()
        .try_for_each(|(i, tx)| append_transaction(&mut document, i + 1, tx))?;

    // Write the document to the output file
    File::create(output)?.write_all(document.as_bytes())?;

    println!("Document rendered to {}", output.display().green());

    Ok(())
}

/// Appends a markdown header for the [MultisigBatch] to the writer.
fn append_header(writer: &mut String, batch: &MultisigBatch) {
    // Header
    writer.push_str(format!("# {}\n", batch.metadata.name).as_ref());

    // Description + Metadata
    writer.push_str(format!("{}\n\n", batch.metadata.description).as_ref());
    writer.push_str(
        format!(
            "The batch will be executed on chain ID `{}`, and contains `{}` transactions.\n",
            batch.chain_id,
            batch.transactions.len()
        )
        .as_ref(),
    );
}

/// Appends a [BatchTransaction] at index `i` to the writer.
fn append_transaction(writer: &mut String, i: usize, tx: &BatchTransaction) -> Result<()> {
    // verify transaction details
    let mut inputs = Vec::new();
    tx.contract_method.inputs.iter().try_for_each(|input| {
        if let Some(param) = tx.contract_inputs_values.get(&input.name) {
            inputs.push(param);
            Ok(())
        } else {
            Err(anyhow!(
                "tx {}. input {} does not exist in the contractInputValues map",
                i,
                input.name
            ))
        }
    })?;
    let function = &tx.contract_method;
    let data: Bytes = encode_function_args(function, inputs)?.into();
    if tx.data != data {
        return Err(anyhow!(
            "tx {}. Tx data does not match contractInputsValues. Instead Got {} instead",
            i,
            data
        ));
    }

    // Newline
    writer.push('\n');

    // Transaction Header
    writer.push_str(format!("## Tx #{}: {}\n", i, tx.metadata.name).as_ref());
    writer.push_str(format!("{}\n", tx.metadata.description).as_ref());

    // Newline
    writer.push('\n');

    // Transaction Details
    writer.push_str(
        format!("**Function Signature:** `{}`\n\n", tx.contract_method.signature()).as_ref(),
    );
    writer.push_str(format!("**To:** `{}`\n\n", tx.to).as_ref());
    writer.push_str(format!("**Value:** `{} WEI`\n\n", tx.value).as_ref());
    writer.push_str(format!("**Raw Input Data:** `{}`\n", tx.data).as_ref());

    // Newline
    writer.push('\n');

    // Transaction Inputs
    writer.push_str("### Inputs\n");
    tx.contract_inputs_values.iter().for_each(|(name, value)| {
        writer.push_str(format!("**{}:** `{}`\n\n", name, value).as_ref());
    });
    Ok(())
}
