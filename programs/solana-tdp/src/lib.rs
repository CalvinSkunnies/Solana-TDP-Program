//! Solana Token Distribution Protocol (TDP)
//!
//! Program ID: Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS
//!
//! Instructions
//! ─────────────────────────────────────────────────
//!  create_stream  – Initialise a vesting stream and lock tokens in a PDA vault
//!  withdraw       – Recipient claims unlocked tokens according to the schedule
//!  cancel         – Sender cancels an active stream; unlocked tokens go to
//!                   recipient, remaining locked tokens are returned to sender

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// ─── Program ────────────────────────────────────────────────────────────────

#[program]
pub mod solana_tdp {
    use super::*;

    /// Create a new token-distribution stream.
    ///
    /// Accounts
    ///   - sender        (signer, mut)      – Stream creator; pays for accounts
    ///   - recipient     (read-only)        – Beneficiary of the stream
    ///   - stream        (init, PDA)        – Stores stream metadata
    ///   - vault         (init, PDA)        – SPL token account that holds locked tokens
    ///   - sender_token  (mut)              – Sender's source token account
    ///   - mint                             – Token mint
    ///   - token_program                    – SPL Token program
    ///   - system_program                   – System program
    ///   - rent                             – Rent sysvar
    ///
    /// Parameters (see CreateStreamParams)
    pub fn create_stream(
        _ctx: Context<CreateStream>,
        _params: CreateStreamParams,
    ) -> Result<()> {
        // TODO: implement stream initialisation logic
        Ok(())
    }

    /// Withdraw vested (unlocked) tokens from an active stream.
    ///
    /// Accounts
    ///   - recipient        (signer, mut)   – Must match stream.recipient
    ///   - stream           (mut, PDA)      – Existing stream account
    ///   - vault            (mut, PDA)      – Locked-token vault
    ///   - recipient_token  (mut)           – Destination token account
    ///   - token_program                    – SPL Token program
    pub fn withdraw(_ctx: Context<Withdraw>) -> Result<()> {
        // TODO: calculate unlocked amount, transfer tokens, update withdrawn total
        Ok(())
    }

    /// Cancel an active stream.
    ///
    /// Accounts
    ///   - sender           (signer, mut)   – Must match stream.sender
    ///   - recipient        (read-only)     – Needed to verify stream ownership
    ///   - stream           (mut, PDA)      – Stream to be cancelled
    ///   - vault            (mut, PDA)      – Locked-token vault
    ///   - sender_token     (mut)           – Return destination for locked remainder
    ///   - recipient_token  (mut)           – Destination for already-vested portion
    ///   - token_program                    – SPL Token program
    pub fn cancel(_ctx: Context<Cancel>) -> Result<()> {
        // TODO: split vault balance between recipient (vested) and sender (unvested)
        Ok(())
    }
}

// ─── Parameters ─────────────────────────────────────────────────────────────

/// Parameters passed to create_stream.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CreateStreamParams {
    /// Total tokens to stream (in raw token units, including decimals).
    pub amount: u64,
    /// Unix timestamp when the stream starts.
    pub start_time: i64,
    /// Unix timestamp when the stream ends (full amount vested).
    pub end_time: i64,
    /// Optional cliff timestamp – no tokens claimable before this point.
    pub cliff_time: i64,
    /// Bump seed stored so the PDA can sign later.
    pub stream_bump: u8,
}

// ─── Account Structs ────────────────────────────────────────────────────────

/// On-chain state for a single vesting stream.
#[account]
#[derive(Debug)]
pub struct StreamAccount {
    /// Public key of the stream creator.
    pub sender: Pubkey,
    /// Public key of the token beneficiary.
    pub recipient: Pubkey,
    /// SPL mint of the distributed token.
    pub mint: Pubkey,
    /// PDA vault that holds the locked tokens.
    pub vault: Pubkey,
    /// Total tokens deposited at stream creation.
    pub amount: u64,
    /// Tokens already withdrawn by the recipient.
    pub amount_withdrawn: u64,
    /// Stream start (Unix timestamp).
    pub start_time: i64,
    /// Stream end / full-vest (Unix timestamp).
    pub end_time: i64,
    /// Cliff timestamp; 0 means no cliff.
    pub cliff_time: i64,
    /// Whether the stream has been cancelled.
    pub cancelled: bool,
    /// PDA bump seed.
    pub bump: u8,
}

impl StreamAccount {
    /// Minimum space required for this account.
    pub const LEN: usize = 8   // discriminator
        + 32   // sender
        + 32   // recipient
        + 32   // mint
        + 32   // vault
        + 8    // amount
        + 8    // amount_withdrawn
        + 8    // start_time
        + 8    // end_time
        + 8    // cliff_time
        + 1    // cancelled
        + 1;   // bump
}

// ─── Contexts ───────────────────────────────────────────────────────────────

#[derive(Accounts)]
#[instruction(params: CreateStreamParams)]
pub struct CreateStream<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    /// CHECK: recipient is stored but not required to sign at creation.
    pub recipient: AccountInfo<'info>,

    #[account(
        init,
        payer = sender,
        space = StreamAccount::LEN,
        seeds = [
            b"stream",
            sender.key().as_ref(),
            recipient.key().as_ref(),
        ],
        bump
    )]
    pub stream: Account<'info, StreamAccount>,

    #[account(
        init,
        payer = sender,
        token::mint = mint,
        token::authority = stream,
        seeds = [
            b"vault",
            stream.key().as_ref(),
        ],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub sender_token: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub recipient: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"stream",
            stream.sender.as_ref(),
            recipient.key().as_ref(),
        ],
        bump = stream.bump,
        has_one = recipient,
    )]
    pub stream: Account<'info, StreamAccount>,

    #[account(
        mut,
        seeds = [b"vault", stream.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub recipient_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    /// CHECK: used only to verify stream ownership seeds.
    pub recipient: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            b"stream",
            sender.key().as_ref(),
            recipient.key().as_ref(),
        ],
        bump = stream.bump,
        has_one = sender,
    )]
    pub stream: Account<'info, StreamAccount>,

    #[account(
        mut,
        seeds = [b"vault", stream.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub sender_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub recipient_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

// ─── Custom Errors ───────────────────────────────────────────────────────────

#[error_code]
pub enum TdpError {
    #[msg("Stream has already been cancelled.")]
    AlreadyCancelled,
    #[msg("Stream has not started yet.")]
    StreamNotStarted,
    #[msg("No tokens are available to withdraw at this time.")]
    NothingToWithdraw,
    #[msg("Start time must be before end time.")]
    InvalidTimeRange,
    #[msg("Cliff time must be between start and end time.")]
    InvalidCliffTime,
    #[msg("Amount must be greater than zero.")]
    InvalidAmount,
}
