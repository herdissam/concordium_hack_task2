//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

/// Your smart contract state.
#[derive(Serialize, SchemaType, Clone)]
pub struct State {
    vote: bool,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
    OwnerError,
    Voted,
}

/// Init function that creates a new smart contract.
#[init(contract = "vote")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    Ok(State {vote: false})
}

type Voting = bool;
/// Receive function. The input parameter is the boolean variable `Voting`.
///  If the account owner does not match the contract owner, the receive function will throw [`Error::OwnerError`].
///  If the number to increment by is not positive or is zero, the receive function will throw [`Error::IncrementError`].
#[receive(
    contract = "vote",
    name = "voting",
    parameter = "bool",
    error = "Error",
    mutable
)]
fn voting<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), Error> {
    // Your code

    let param: Voting = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::OwnerError
    );

    ensure!(param == false, Error::Voted);
    state.vote = param;
    Ok(())
    
}

/// View function that returns the content of the state.
#[receive(contract = "vote", name = "view", return_value = "bool")]
fn view<'b, S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &'b impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<bool> {
    Ok(host.state().vote)
}

#[concordium_cfg_test]
mod tests {
    use super::*;
    use test_infrastructure::*;

    type ContractResult<A> = Result<A, Error>;

    #[concordium_test]
    /// Test that initializing the contract succeeds with some state.
    fn test_init() {
        let ctx = TestInitContext::empty();

        let mut state_builder = TestStateBuilder::new();

        let state_result = init(&ctx, &mut state_builder);
        state_result.expect_report("Contract initialization results in error");
    }

    #[concordium_test]
    /// Test that invoking the `voting` endpoint with the `false` parameter
    /// succeeds in updating the contract.
    fn test_throw_no_error() {
        let ctx = TestInitContext::empty();

        let mut state_builder = TestStateBuilder::new();

        // Initializing state
        let initial_state = init(&ctx, &mut state_builder).expect("Initialization should pass");

        let mut ctx = TestReceiveContext::empty();

        let throw_error = false;
        let parameter_bytes = to_bytes(&throw_error);
        ctx.set_parameter(&parameter_bytes);

        let mut host = TestHost::new(initial_state, state_builder);

        // Call the contract function.
        let result: ContractResult<()> = voting(&ctx, &mut host);

        // Check the result.
        claim!(result.is_ok(), "Results in rejection");
    }

    #[concordium_test]
    /// Test that invoking the `voting` endpoint with the `true` parameter
    /// results in the `Voted` being thrown.
    fn test_throw_error() {
        let ctx = TestInitContext::empty();

        let mut state_builder = TestStateBuilder::new();

        // Initializing state
        let initial_state = init(&ctx, &mut state_builder).expect("Initialization should pass");

        let mut ctx = TestReceiveContext::empty();

        let throw_error = true;
        let parameter_bytes = to_bytes(&throw_error);
        ctx.set_parameter(&parameter_bytes);

        let mut host = TestHost::new(initial_state, state_builder);

        // Call the contract function.
        let error: ContractResult<()> = voting(&ctx, &mut host);

        // Check the result.
        claim_eq!(error, Err(Error::Voted), "Function should throw an error.");
    }
}