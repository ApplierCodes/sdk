use anchor_lang::prelude::*;

declare_id!("ApPLiEr111111111111111111111111111111111");

#[program]
pub mod applier {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.authority = ctx.accounts.authority.key();
        Ok(())
    }

    // Verify identity (by authority)
    pub fn verify_identity(ctx: Context<VerifyIdentity>) -> Result<()> {
        let identity = &mut ctx.accounts.identity;
        identity.user = ctx.accounts.user.key();
        identity.verified = true;
        identity.verifier = ctx.accounts.authority.key();
        Ok(())
    }

    // Submit application (only verified users)
    pub fn submit_application(
        ctx: Context<SubmitApplication>,
        data_uri: String,
    ) -> Result<()> {
        require!(
            ctx.accounts.identity.verified,
            ApplierError::IdentityNotVerified
        );

        let app = &mut ctx.accounts.application;
        app.applicant = ctx.accounts.user.key();
        app.data_uri = data_uri;
        app.approved = false;

        Ok(())
    }

    // Approve application
    pub fn approve_application(ctx: Context<ApproveApplication>) -> Result<()> {
        ctx.accounts.application.approved = true;
        Ok(())
    }
}
