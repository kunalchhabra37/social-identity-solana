use anchor_lang::prelude::*;

declare_id!("HmoyrnYPqW5Rq5bhQfT8CCcstwpKize2dUiFooziyipe");

#[program]
pub mod social_identity {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, description: String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.name = name;
        user_account.description = description;
        user_account.bump = *ctx.bumps.get("user_account").unwrap();
        user_account.types = vec![];

        Ok(())
    }

    pub fn add(ctx: Context<Add>, type_info: String, url: String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let type_info_account = &mut ctx.accounts.type_info_account;
        user_account.types.push(type_info.clone());
        type_info_account.url = url;
        type_info_account.type_info = type_info;
        type_info_account.bump = *ctx.bumps.get("type_info_account").unwrap();

        Ok(())
    }

    pub fn delete(ctx: Context<Delete>, type_info:String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;

        user_account.types.retain(|x| x != &type_info);

        Ok(())
    }

    pub fn edit_url(ctx: Context<EditUrl>, url:String) -> Result<()> {
        let type_info_account = &mut ctx.accounts.type_info_account;

        type_info_account.url = url;

        Ok(())
    }

    pub fn edit(ctx: Context<Edit>, type_info: String, type_info_old: String, url: String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let type_info_account = &mut ctx.accounts.type_info_account;
        user_account.types.retain(|x| x != &type_info_old);

        user_account.types.push(type_info.clone());

        type_info_account.url = url;
        type_info_account.type_info = type_info;

        Ok(())
    }
}

// close

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 1024,
        seeds = [b"user-account", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(type_info: String)]
pub struct Add<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 300,
        seeds = [b"type-info", user.key().as_ref(), user_account.key().as_ref(), type_info.as_ref()], 
        bump
    )]
    pub type_info_account: Account<'info, TypeInfoAccount>,

    #[account(
        mut,
        seeds = [b"user-account", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(
        mut,
        seeds = [b"type-info", user.key().as_ref(), user_account.key().as_ref(), type_info_account.type_info.as_ref()], 
        bump = type_info_account.bump,
        close = user
    )]
    pub type_info_account: Account<'info, TypeInfoAccount>,

    #[account(
        mut,
        seeds = [b"user-account", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EditUrl<'info> {
    #[account(
        mut,
        seeds = [b"type-info", user.key().as_ref(), user_account.key().as_ref(), type_info_account.type_info.as_ref()], 
        bump = type_info_account.bump
    )]
    pub type_info_account: Account<'info, TypeInfoAccount>,

    #[account(
        mut,
        seeds = [b"user-account", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(type_info: String)]
pub struct Edit<'info> {
    #[account(
        mut,
        seeds = [b"user-account", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds = [b"type-info", user.key().as_ref(), user_account.key().as_ref(), type_info_account_old.type_info.as_ref()], 
        bump = type_info_account_old.bump,
        close = user
    )]
    pub type_info_account_old: Account<'info, TypeInfoAccount>,

    #[account(
        init,
        payer = user,
        space = 8 + 300,
        seeds = [b"type-info", user.key().as_ref(), user_account.key().as_ref(), type_info.as_ref()], 
        bump
    )]
    pub type_info_account: Account<'info, TypeInfoAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserAccount {
    pub name: String,
    pub description: String,
    pub types: Vec<String>,
    bump: u8,
}

#[account]
pub struct TypeInfoAccount {
    pub url: String,
    type_info: String,
    bump: u8,
}