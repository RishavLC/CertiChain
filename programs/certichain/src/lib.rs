use anchor_lang::prelude::*;

declare_id!("7Y5G9cN5ormvwdQkxYwqTp4h5ENBXN44hjnuTzN3FhED");

#[program]
pub mod certichain {
    use super::*;

    pub fn issue_certificate(
        ctx: Context<IssueCertificate>,
        certificate_id: String,
        student_name: String,
        course_name: String,
    ) -> Result<()> {
        let cert = &mut ctx.accounts.certificate;
        cert.certificate_id = certificate_id;
        cert.student_name = student_name;
        cert.course_name = course_name;
        cert.issuer = ctx.accounts.issuer.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IssueCertificate<'info> {
    #[account(init, payer = issuer, space = 8 + 256)]
    pub certificate: Account<'info, Certificate>,
    #[account(mut)]
    pub issuer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Certificate {
    pub certificate_id: String,
    pub student_name: String,
    pub course_name: String,
    pub issuer: Pubkey,
}
