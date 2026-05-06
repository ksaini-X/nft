use anchor_lang::{
    prelude::{Instructions, Pubkey},
    solana_program::{self, system_program},
    AnchorDeserialize, Discriminator, ToAccountMetas,
};
use anchor_spl::associated_token::spl_associated_token_account::instruction;
use nft::{
    instructions::Initialize,
    state::{Config, Whitelist},
};
use solana_message::AccountMeta;
use solana_transaction::Transaction;
use {litesvm::LiteSVM, solana_keypair::Keypair, solana_signer::Signer};

use anchor_lang::system_program::ID as system_program_id;

#[test]
fn test_config_init() {
    let mut svm = LiteSVM::new();

    let program_id = nft::id();
    let bytes = include_bytes!("../../../target/deploy/nft.so");
    svm.add_program(program_id, bytes).unwrap();

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let (config, _) = Pubkey::find_program_address(&[b"config".as_ref()], &program_id);

    let disc = nft::instruction::Initialize::DISCRIMINATOR;
    let mut ins_data = Vec::new();
    ins_data.extend_from_slice(&disc);
    ins_data.extend_from_slice(&(10_000u64).to_le_bytes());
    ins_data.extend_from_slice(&(100u32).to_le_bytes());

    let ix = solana_program::instruction::Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(config, false),
            AccountMeta::new(system_program_id, false),
        ],
        data: ins_data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let _ = svm.send_transaction(tx);

    let config_account = svm.get_account(&config).expect("Config should exist");
    println!("{:?}", config_account);

    let config_acc_data = Config::deserialize(&mut &config_account.data[8..]).expect("Failed");
    println!("{:?}", config_acc_data);

    assert_eq!(config_acc_data.authority, payer.pubkey(), "Mismatch");
    assert_eq!(config_acc_data.price, 10_000, "Mismatch");
    assert_eq!(config_acc_data.max_supply, 100, "Mismatch");
    assert_eq!(config_acc_data.current_supply, 0, "Mismatch");

    println!("Passed")
}

#[test]
fn test_whitelist() {
    let mut svm = LiteSVM::new();

    let program_id = nft::id();
    let bytes = include_bytes!("../../../target/deploy/nft.so");
    svm.add_program(program_id, bytes).unwrap();

    let payer = Keypair::new();
    let user = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&user.pubkey(), 1_000_000_000).unwrap();

    let (config, _) = Pubkey::find_program_address(&[b"config".as_ref()], &program_id);

    let disc = nft::instruction::Initialize::DISCRIMINATOR;
    let mut ins_data = Vec::new();
    ins_data.extend_from_slice(&disc);
    ins_data.extend_from_slice(&(10_000u64).to_le_bytes());
    ins_data.extend_from_slice(&(100u32).to_le_bytes());

    let ix = solana_program::instruction::Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(config, false),
            AccountMeta::new(system_program_id, false),
        ],
        data: ins_data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let _ = svm.send_transaction(tx);

    let (whitelist, _) = Pubkey::find_program_address(
        &[b"whitelist".as_ref(), user.pubkey().as_ref()],
        &program_id,
    );

    let whitelist_disc = nft::instruction::Whitelist::DISCRIMINATOR;

    let mut whitelist_ins_data: Vec<u8> = Vec::new();
    whitelist_ins_data.extend_from_slice(&whitelist_disc);

    let whitelist_ix = solana_program::instruction::Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(user.pubkey(), false),
            AccountMeta::new(config, false),
            AccountMeta::new(whitelist, false),
            AccountMeta::new(system_program_id, false),
        ],
        data: whitelist_ins_data,
    };

    let whitelist_tx = Transaction::new_signed_with_payer(
        &[whitelist_ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let _ = svm.send_transaction(whitelist_tx);

    let whitelist_account = svm.get_account(&whitelist).expect("Whitelist should exist");
    println!("{:?}", whitelist_account);

    let whitelist_acc_data =
        Whitelist::deserialize(&mut &whitelist_account.data[8..]).expect("Failed");
    println!("{:?}", whitelist_acc_data);

    assert_eq!(whitelist_acc_data.user, user.pubkey(), "Mismatch");
    assert_eq!(whitelist_acc_data.has_minted, false, "Mismatch");

    println!("Passed")
}

// #[test]
fn test_remove_whitelist() {
    let mut svm = LiteSVM::new();

    let program_id = nft::id();
    let bytes = include_bytes!("../../../target/deploy/nft.so");
    svm.add_program(program_id, bytes).unwrap();

    let payer = Keypair::new();
    let user = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let (config, _) = Pubkey::find_program_address(&[b"config".as_ref()], &program_id);
    let (whitelist, _) = Pubkey::find_program_address(
        &[b"whitelist".as_ref(), user.pubkey().as_ref()],
        &program_id,
    );

    let disc = nft::instruction::Whitelist::DISCRIMINATOR;

    let mut ins_data = Vec::new();
    ins_data.extend_from_slice(&disc);

    let ix = solana_program::instruction::Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(user.pubkey(), false),
            AccountMeta::new(config, false),
            AccountMeta::new(whitelist, false),
            AccountMeta::new(system_program_id, false),
        ],
        data: ins_data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let _ = svm.send_transaction(tx);

    let whitelist_account = svm.get_account(&whitelist).expect("Whitelist should exist");
    println!("{:?}", whitelist_account);

    let whitelist_acc_data =
        Whitelist::deserialize(&mut &whitelist_account.data[8..]).expect("Failed");
    println!("{:?}", whitelist_acc_data);

    assert_eq!(whitelist_acc_data.user, user.pubkey(), "Mismatch");
    assert_eq!(whitelist_acc_data.has_minted, false, "Mismatch");

    let remove_disc = nft::instruction::RemoveWhitelist::DISCRIMINATOR;
    let mut rm_ins_data: Vec<u8> = Vec::new();
    rm_ins_data.extend_from_slice(&remove_disc);

    let remove_ix = solana_program::instruction::Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(config, false),
            AccountMeta::new(whitelist, false),
            AccountMeta::new(system_program_id, false),
        ],
        data: rm_ins_data,
    };

    let rm_tx = Transaction::new_signed_with_payer(
        &[remove_ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let _ = svm.send_transaction(rm_tx);

    let whitelist_account = svm
        .get_account(&whitelist)
        .expect("Whitelist should not exist");
}
