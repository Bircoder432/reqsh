use crate::{display::display_response, request::Request, runner::fetch, state::ShellState};

pub fn execute(req: Request, ctx: &ShellState) -> Result<String, String> {
    let base_url = ctx.get_base_url();
    let global_headers = ctx.get_headers();

    let response = fetch(&req, base_url, global_headers);

    match response {
        Ok(r) => Ok(display_response(r)),
        Err(e) => Err(e),
    }
}
