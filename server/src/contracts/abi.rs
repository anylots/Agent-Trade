use alloy::sol;
use serde::{Deserialize, Serialize};

sol! {
    #[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
    #[sol(rpc)]
    interface IERC20 {
        function transfer(address to, uint256 amount) public returns (bool);
        function decimals() public view returns (uint8);
        function allowance(address owner, address spender) external view returns (uint256);
        function approve(address spender, uint256 amount) external returns (bool);
        function balanceOf(address owner) external view returns (uint256);
    }
}
