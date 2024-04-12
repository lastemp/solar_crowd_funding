// admin instructions
//pub mod approve_tree_owner;
pub mod init;

// public instructions
pub mod invest_in_project;
pub mod register_institution;
pub mod register_investor;
/* pub mod purchase_carbon_credits;
pub mod register_institution;
pub mod register_tree_owner;
pub mod withdraw_tree_owner_funds; */

// bring everything in scope
pub use {
    init::*,
    invest_in_project::*,
    register_institution::*,
    //approve_tree_owner::*, init::*, purchase_carbon_credits::*, register_institution::*,
    //register_tree_owner::*, withdraw_tree_owner_funds::*,
    register_investor::*,
};
