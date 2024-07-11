use chat::Solver;
use cliclack::Select;

#[tokio::main]
async fn main() {
    let mut solver = Solver::new();
    match Select::new("Operation")
        .items(&[(0, "Ask", ""), (1, "Config", "")])
        .interact()
        .unwrap()
    {
        0 => solver.run().await,
        1 => solver.set_config(),
        _ => (),
    }
}
