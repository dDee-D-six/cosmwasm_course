#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary,Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError,Uint64};
use std::ops::Add;
use cosmwasm_std::attr;
use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{DEE_COUNT};

// version info for migration info
// const CONTRACT_NAME: &str = "crates.io:dee-cosm";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    DEE_COUNT.save(deps.storage, &Uint64::zero())?;
    let res = Response::new();
    let _ = info;
    let _ = msg;
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    // info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let mut count = Uint64::zero();
    match msg{
        ExecuteMsg:: Dee{} =>{
           let _:Result<Uint64,StdError> = DEE_COUNT.update(deps.storage, |exists|{
                count = exists.add(Uint64::from(1u8));
                Ok(count)
            });
          let mut res = Response::new();
          res.attributes.push(attr("dee_count",count));
          res.attributes.push(attr("dee_count","haha"));
          res.data = Some(to_binary(&count)?);
          Ok(res)
        }
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    let count:Uint64 = DEE_COUNT.load(deps.storage)?;
    to_binary(&count)
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info, mock_dependencies};
    use cosmwasm_std::{coins, from_binary};

    
    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator",&[]);

        instantiate(deps.as_mut(),mock_env(), info.clone(),msg.clone()).unwrap();
 
        // we can just call .unwrap() to assert this was a success
        let msg = ExecuteMsg::Dee{};
        let res = execute(deps.as_mut(),mock_env(), msg.clone()).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: Uint64 = from_binary(&res).unwrap();
        assert_eq!(Uint64::zero(), value);
    }

    #[test]
    fn test_dee(){
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg{};
        let info = mock_info("creator",&[]);

        instantiate(deps.as_mut(),mock_env(),info.clone(),msg.clone()).unwrap();
        
        let msg = ExecuteMsg::Dee{};
        let res = (deps.as_mut(),mock_env(),info.clone(),msg.clone()).unwrap();
        assert_eq!(res.attributes.len(),1);
        assert_eq!(res.attributes. vec![attr(dee_count, 1)]);
        let data:String = from_binary(value: &Binary)
    }
    /*
    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }
    */

/*
    #[test]
    fn reset() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
*/
}
