use cosmwasm_std::{ DepsMut, Env, MessageInfo, Response };

use crate::{ ContractError, state::{ POLLS, Poll, USER_VOTED } };

pub fn create_poll(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    question: String
) -> Result<Response, ContractError> {
    if POLLS.has(deps.storage, question.clone()) {
        return Err(ContractError::CustomError { val: "Question already exists!".to_string() });
    }

    let poll = Poll {
        question: question.clone(),
        yes_votes: 0,
        no_votes: 0,
    };

    POLLS.save(deps.storage, question, &poll)?;

    Ok(Response::new().add_attribute("action", "create_poll"))
}

pub fn vote(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    question: String,
    choice: bool
) -> Result<Response, ContractError> {
    if !POLLS.has(deps.storage, question.clone()) {
        return Err(ContractError::CustomError { val: "No such question exists".to_string() });
    }

    if USER_VOTED.has(deps.storage, (question.clone(), info.sender.clone())) {
        return Err(ContractError::CustomError { val: "User has already voted".to_string() });
    }

    USER_VOTED.save(deps.storage, (question.clone(), info.sender), &true)?;

    let mut poll = POLLS.load(deps.storage, question.clone())?;

    if choice {
        poll.yes_votes += 1;
    } else {
        poll.no_votes += 1;
    }

    POLLS.save(deps.storage, question, &poll)?;

    Ok(Response::new().add_attribute("action", "vote"))
}