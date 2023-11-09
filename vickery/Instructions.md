# Auction Library Assignment

## Repository Setup

1. **Create a new GitHub Repository**

2. **Initialize a Foundry Project**

3. **Copy the skeleton contracts in this directory into your foundry project:**

## Contract Implementation

4. **Skeletal Code:**

   - Utilize the provided skeletal implementation of the contract code found in the `BasicVickreyAuction.sol` file.

5. **Test Cases:**

   - Develop comprehensive test cases to thoroughly evaluate your contract's functionalities.

6. **Coverage Report:**
   - Generate a Solidity coverage report to ensure 100% line and branch coverage of your contract code.

## Submission

**GitHub Repository:** - Provide the link to your GitHub repository, which should contain: - The modified `VickreyAuction` contract. - Your comprehensive test cases. - The coverage report demonstrating 100% line and branch coverage.

## Milestone #1 Tasks

1. **Seller Setup:**

   - Implement the `createAuction` function, allowing a seller to list a physical item for auction. This function should permit the seller to set parameters like `startTime`, `bidPeriod`, `revealPeriod`, and `reservePrice`.
   - The auction begins at the block in which the contract is created.

2. **Bidding:**

   - Implement the `commitBid` function, which permits interested bidders to commit their bids for an item with the price of their choice. Bidders should send Ether as collateral. Ensure that bidders can only commit bids after the `startTime` set by the seller has passed and until the start of the `revealPeriod`.

3. **Bid Reveal:**

   - Implement the `revealBid` function. Bidders should be able to reveal their bids during the `revealPeriod` using the `bidValue` and a `nonce`. Prevent bids from being submitted after this period.

4. **Auction Completion:**

   - Implement the `endAuction` function, allowing the auction to be completed after all bidders have revealed their bids. Ensure that the winning bidder is determined, the asset is transferred, collateral is managed, and cases where the reserve price is not exceeded are handled.

5. **Withdraw Collateral:**
   - Implement the `withdrawCollateral` function, enabling non-winning bidders to withdraw their bid collateral after the auction ends.
