use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod escrow_release {
    use super::*;

    pub fn release(ctx: Context<Release>) -> Result<()> {
        let authority = ctx.accounts.authority.key();
        let seller = ctx.accounts.seller.key();
        let amount = apply_release(&mut ctx.accounts.escrow, authority, seller)?;
        transfer_lamports(
            &ctx.accounts.escrow.to_account_info(),
            &ctx.accounts.seller.to_account_info(),
            amount,
        )?;
        Ok(())
    }
}

pub fn apply_release(escrow: &mut Escrow, authority: Pubkey, seller: Pubkey) -> Result<u64> {
    let _ = (escrow, authority, seller);
    todo!("implement apply_release");
}

fn transfer_lamports<'info>(
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let from_balance = from.lamports();
    let to_balance = to.lamports();
    let new_from = from_balance
        .checked_sub(amount)
        .ok_or_else(|| error!(EscrowError::InsufficientFunds))?;
    let new_to = to_balance
        .checked_add(amount)
        .ok_or_else(|| error!(EscrowError::Overflow))?;
    **from.try_borrow_mut_lamports()? = new_from;
    **to.try_borrow_mut_lamports()? = new_to;
    Ok(())
}

#[account]
pub struct Escrow {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub arbiter: Pubkey,
    pub amount: u64,
    pub status: Status,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Funded,
    Released,
    Refunded,
}

#[derive(Accounts)]
pub struct Release<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
    /// CHECK: apply_release requires this key to equal escrow.seller
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
}

#[error_code]
pub enum EscrowError {
    NotAuthorized,
    BadStatus,
    SellerMismatch,
    InsufficientFunds,
    Overflow,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;

    fn key(byte: u8) -> Pubkey {
        let mut bytes = [0u8; 32];
        bytes[0] = byte;
        Pubkey::new_from_array(bytes)
    }

    fn funded() -> Escrow {
        Escrow {
            buyer: key(1),
            seller: key(2),
            arbiter: key(3),
            amount: 500,
            status: Status::Funded,
        }
    }

    #[test]
    fn buyer_releases() {
        let mut escrow = funded();
        let amount = apply_release(&mut escrow, key(1), key(2)).unwrap();
        assert_eq!(amount, 500);
        assert_eq!(escrow.status, Status::Released);
        assert_eq!(escrow.amount, 0);
    }

    #[test]
    fn arbiter_releases() {
        let mut escrow = funded();
        let amount = apply_release(&mut escrow, key(3), key(2)).unwrap();
        assert_eq!(amount, 500);
        assert_eq!(escrow.status, Status::Released);
    }

    #[test]
    fn seller_is_rejected() {
        let mut escrow = funded();
        let err = apply_release(&mut escrow, key(2), key(2)).unwrap_err();
        assert_eq!(err.error_code_number(), anchor_lang::error!(EscrowError::NotAuthorized).error_code_number());
        assert_eq!(escrow.status, Status::Funded);
        assert_eq!(escrow.amount, 500);
    }

    #[test]
    fn refunded_is_rejected() {
        let mut escrow = funded();
        escrow.status = Status::Refunded;
        let err = apply_release(&mut escrow, key(1), key(2)).unwrap_err();
        assert_eq!(err.error_code_number(), anchor_lang::error!(EscrowError::BadStatus).error_code_number());
        assert_eq!(escrow.amount, 500);
    }

    #[test]
    fn second_release_is_rejected() {
        let mut escrow = funded();
        apply_release(&mut escrow, key(1), key(2)).unwrap();
        let err = apply_release(&mut escrow, key(1), key(2)).unwrap_err();
        assert_eq!(err.error_code_number(), anchor_lang::error!(EscrowError::BadStatus).error_code_number());
        assert_eq!(escrow.amount, 0);
    }

    #[test]
    fn seller_mismatch_leaves_state() {
        let mut escrow = funded();
        let err = apply_release(&mut escrow, key(1), key(9)).unwrap_err();
        assert_eq!(err.error_code_number(), anchor_lang::error!(EscrowError::SellerMismatch).error_code_number());
        assert_eq!(escrow.status, Status::Funded);
        assert_eq!(escrow.amount, 500);
    }
}
