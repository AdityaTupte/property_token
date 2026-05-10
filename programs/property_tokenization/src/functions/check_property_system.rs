
use anchor_lang::prelude::*;

use crate::{errors::ErrorCode, state::PropertySystemAccount};



pub fn check_property_system(
    system : & Account<PropertySystemAccount>,
)->Result<()>{

    require!(system.ready_for_listing,ErrorCode::PropertySystemIsNotReady);

    Ok(())
}
