use crate::types::context::Context;

pub fn get_book(ctx: Context) {
    let res = String::from("Create book called");
    ctx.handle_write(res);
}

pub fn create_book(ctx: Context) {
    let res = String::from("Create book called");
    ctx.handle_write(res);
}
