#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{ mock_dependencies, mock_env, mock_info },
        Attribute,
        from_binary,
    };

    use crate::{
        msg::{ InstantiateMsg, ExecuteMsg, QueryMsg, GetPollResponse },
        contract::{ execute, instantiate, query },
        ContractError,
        state::Poll,
    };

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: String::from("addr1"),
        };

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.attributes, vec![Attribute::new("action", "instantiate")]);
    }

    #[test]
    fn test_create_poll() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);

        let msg = InstantiateMsg {
            admin_address: String::from("addr1"),
        };

        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let question = String::from("Have you read a book?");
        let msg = ExecuteMsg::CreatePoll { question };

        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.attributes, vec![Attribute::new("action", "create_poll")]);
    }

    // should fail as creating 2 polls with same question
    #[test]
    fn test_create_poll_should_fail() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);

        let msg = InstantiateMsg {
            admin_address: String::from("addr1"),
        };

        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let question = String::from("Have you read a book");
        let msg = ExecuteMsg::CreatePoll { question };

        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();

        let question = String::from("Have you read a book");
        let msg = ExecuteMsg::CreatePoll { question };

        let res = execute(deps.as_mut(), env, info, msg).unwrap_err();
        match res {
            ContractError::CustomError { val } => {
                assert_eq!(val, "Question already exists!".to_string())
            }
            _ => {}
        }
    }

    #[test]
    fn test_vote_poll() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);

        let msg = InstantiateMsg {
            admin_address: String::from("addr1"),
        };

        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let question = String::from("Have you read a book?");
        let msg = ExecuteMsg::CreatePoll { question };

        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::Vote { question: "Have you read a book?".to_string(), choice: true };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        assert_eq!(res.attributes, vec![Attribute::new("action", "vote")]);
    }

    // should fail as question does not exist
    #[test]
    fn test_vote_poll_fails_question_does_not_exist() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);

        let msg = InstantiateMsg {
            admin_address: String::from("addr1"),
        };

        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::Vote { question: "Have you read a book?".to_string(), choice: true };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

        match res {
            ContractError::CustomError { val } =>
                assert_eq!(val, "No such question exists".to_string()),
            _ => panic!(),
        }
    }

    // should fail as user already voted
    #[test]
    fn test_vote_poll_fails_user_already_voted() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);

        let msg = InstantiateMsg {
            admin_address: String::from("addr1"),
        };

        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let question = String::from("Have you read a book?");
        let msg = ExecuteMsg::CreatePoll { question };

        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::Vote { question: "Have you read a book?".to_string(), choice: false };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::Vote { question: "Have you read a book?".to_string(), choice: true };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

        match res {
            ContractError::CustomError { val } =>
                assert_eq!(val, "User has already voted".to_string()),
            _ => panic!(),
        }
    }

    #[test]
    fn test_get_poll() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);

        let msg = InstantiateMsg {
            admin_address: String::from("addr1"),
        };

        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let question = String::from("Have you read a book?");
        let msg = ExecuteMsg::CreatePoll { question: question.clone() };

        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let msg = QueryMsg::GetPoll { question: question.clone() };
        let res = query(deps.as_ref(), env, msg).unwrap();
        let get_polls_response: GetPollResponse = from_binary(&res).unwrap();

        assert_eq!(get_polls_response, GetPollResponse {
            poll: Some(Poll {
                question,
                yes_votes: 0,
                no_votes: 0,
            }),
        })
    }

    // No poll created. So get fails.
    #[test]
    fn test_get_poll_fails() {
        let deps = mock_dependencies();
        let env = mock_env();
        let msg = QueryMsg::GetPoll { question: "Hey".to_string() };
        let res = query(deps.as_ref(), env, msg).unwrap();
        let get_polls_response: GetPollResponse = from_binary(&res).unwrap();

        assert_eq!(get_polls_response, GetPollResponse {
            poll: None,
        })
    }
}