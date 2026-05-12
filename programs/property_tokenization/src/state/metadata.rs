use anchor_lang::prelude::*;

#[account]

pub struct Metadata {

    pub symbol:String,

    pub name:String,

    pub uri:String,

}