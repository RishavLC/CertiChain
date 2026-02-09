use anchor_lang::prelude::*;

declare_id!("PeP9DUUhKve9zvWUM1dpRzWNFWpWjdYhUAcpK8bKJx6");

#[program]
pub mod certichain {
    use super::*;

    pub fn issue_certificate(
        ctx: Context<IssueCertificate>,
        student_name: String,
        course: String,
        college: String,
    ) -> Result<()> {
        let cert = &mut ctx.accounts.certificate;
        cert.student_name = student_name;
        cert.course = course;
        cert.college = college;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IssueCertificate<'info> {
    #[account(init, payer = user, space = 256)]
    pub certificate: Account<'info, Certificate>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Certificate {
    pub student_name: String,
    pub course: String,
    pub college: String,
}
