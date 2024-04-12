use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Project {
    #[max_len(10)]
    pub project_reference: String, // unique reference of the project
    #[max_len(30)]
    pub project_name: String, // project name
    #[max_len(3)]
    pub country: String, // home country where project is implemented
    pub active: bool,               // status of project
    pub project_funds: u32,         // funds needed for completion of project
    pub investor_funds_raised: u32, // funds raised by investors
    pub is_initialized: bool,       // is project initiated
    pub is_launched: bool,          // is project launched
    #[max_len(5)]
    pub investors: Vec<Pubkey>, // list of the investors
}
