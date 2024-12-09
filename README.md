# lst_program
I orginally made a LST project for a client using Solana native, however I wanted to try it out in Anchor as well. (https://github.com/btorressz/LST)

Dev net deployment: (https://explorer.solana.com/address/64pMzn8nuvgJ9ja7gh2hV6wpg5Jt7xoReLxjVJkcwY5k?cluster=devnet)

# LST Program: Liquid Staking Token(lst_program)

This **LST Program** is a Solana Program (smart contract) designed to facilitate liquid staking on the Solana blockchain. Users can stake their SOL tokens and receive **Liquidity Staking Tokens (LST)** as proof of their stake. LST tokens are fungible and can be utilized across the Solana ecosystem while users continue earning staking rewards.

---

## Overview

The program supports:
- **Liquid Staking**: Stake SOL and receive LST tokens for liquidity.
- **Withdrawals**: Redeem LST tokens to withdraw SOL.
- **Compounding Rewards**: Automatically compound rewards for the pool.
- **Admin Controls**: Manage fees, redelegate stakes, and control program states.

---

## Features 

### 1. **Initialize Pool**
Sets up the staking pool with an admin and initial parameters.
- **Admin**: Defines the account managing the pool.
- **Fee Rate**: A fee percentage applied to staking and withdrawals.
- The pool starts unpaused and ready for staking.

---

### 2. **Stake SOL**
Allows users to stake SOL into the pool and receive LST tokens.
- Calculates a fee deducted from the staked amount.
- Mints LST tokens equivalent to the net staked SOL.
- Updates the pool's state to track total staked and minted amounts.
- Transfers the fee to the admin's account.

---

### 3. **Withdraw SOL**
Enables users to redeem LST tokens for SOL.
- Burns the specified amount of LST tokens.
- Deducts a fee from the withdrawn SOL.
- Updates the pool's state to reflect the reduced staked and minted amounts.
- Transfers the net SOL back to the user and the fee to the admin.

---

### 4. **Auto-Compound Rewards**
Automatically calculates and compounds rewards for all staked SOL.
- Adds rewards back into the total staked amount.
- Increases the pool's reward tracking values.
- Emits an event for transparency.

---

### 5. **Redelegate**
Allows the admin to redelegate staked SOL to a different validator.
- Useful for optimizing validator performance or yield.
- Updates validator information and emits a redelegate event.

---

### 6. **Admin Update**
Updates the admin account managing the pool.
- Transfers admin rights to a new public key.
- Emits an event recording the change.

---

### 7. **Pause Program**
Toggles the program between active and paused states.
- Paused state disables staking and withdrawing operations.
- Useful for maintenance or emergencies.
- Emits an event reflecting the pause status.

---

### 8. **Get Pool Statistics**
Provides real-time information about the pool’s state:
- Total SOL staked.
- Total LST tokens minted.
- Rewards compounded.
- Current fee rate.

---

## Events

The program uses events to provide real-time updates for key actions:

| **Event**            | **Description**                                         |
|-----------------------|---------------------------------------------------------|
| `StakeEvent`         | Logs details when a user stakes SOL.                    |
| `WithdrawEvent`      | Logs details when a user withdraws SOL.                 |
| `CompoundRewardsEvent` | Logs the amount of rewards compounded.                 |
| `RedelegateEvent`    | Logs when a redelegation occurs.                        |
| `AdminUpdateEvent`   | Logs when the admin is updated.                         |
| `PauseEvent`         | Logs when the program is paused or resumed.             |

---

## Accounts

### **PoolState**
Tracks the state of the staking pool.

| **Field**            | **Type**     | **Description**                            |
|-----------------------|--------------|--------------------------------------------|
| `admin`              | `Pubkey`     | Public key of the pool admin.              |
| `total_staked`       | `u64`        | Total amount of staked SOL.                |
| `total_minted`       | `u64`        | Total LST tokens minted.                   |
| `rewards_compounded` | `u64`        | Total rewards added to the pool.           |
| `fee_basis_points`   | `u16`        | Fee percentage in basis points (1% = 100). |
| `paused`             | `bool`       | Whether the program is currently paused.   |

---

## Error Codes

| **Error Code** | **Description**                  |
|-----------------|----------------------------------|
| `Paused`       | The program is currently paused. |

---

## License
This project is under the **MIT LICENSE**
