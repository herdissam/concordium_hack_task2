//! # A Concordium V1 smart contract
use concordium_std::{collections::BTreeMap, *};
use core::fmt::Debug;

type VotingOption = String;
type VotingIndex = u32;

#[derive(Serialize, SchemaType, Clone)]
pub struct State {
    description: String,
    options: Vec<VotingOption>,
    end_time: Timestamp,
    ballots: BTreeMap<AccountAddress, VotingIndex>,
}

#[derive(Serialize, SchemaType)]
struct  InitParameters {
    description: String,
    options: Vec<VotingOption>,
    end_time: Timestamp,
}

#[init(contract = "firstproject", parameter = "InitParameter")]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    let param: InitParameters = ctx.parameter_cursor().get()?;

    Ok(State {
        description: param.description,
        options: param.options,
        end_time: param.end_time,
        ballots: BTreeMap::new(),
    })
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum ContractError{
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
    VotingFinished,
    ContractVoters,
}

#[receive(
    contract = "firstproject",
    name = "vote",
    parameter = "VotingOption",
    error = "ContractError",
    mutable
)]
fn receive<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), ContractError> {
    // ensure that end_time hasn't been reached
    if host.state().end_time < ctx.metadata().slot_time(){
        return  Err(ContractError::VotingFinished);
    }
    // ensure that the sender is an account (not a contract)
    let acc: AccountAddress = match ctx.sender() {
        Address::Account(acc) => acc,
        Address::Contract(_) => return Err(ContractError::ContractVoters),
    };
    // ensure that the voting option is valid
    let voting_option: VotingOption = ctx.parameter_cursor().get()?;
    let voting_index = match host.state().options.iter().position(|option| *option == voting_option) {
        
    }
    // add/update the vote
    // return OK
    Ok(())
}

/// View function that returns the content of the state.
#[receive(contract = "firstproject", name = "view", return_value = "State")]
fn view<'a, 'b, S: HasStateApi>(
    _ctx: &'a impl HasReceiveContext,
    host: &'b impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<&'b State> {
    Ok(host.state())
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
    /// Test that invoking the `receive` endpoint with the `false` parameter
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
        let result: ContractResult<()> = receive(&ctx, &mut host);

        // Check the result.
        claim!(result.is_ok(), "Results in rejection");
    }

    #[concordium_test]
    /// Test that invoking the `receive` endpoint with the `true` parameter
    /// results in the `YourError` being thrown.
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
        let error: ContractResult<()> = receive(&ctx, &mut host);

        // Check the result.
        claim_eq!(error, Err(Error::YourError), "Function should throw an error.");
    }
}
