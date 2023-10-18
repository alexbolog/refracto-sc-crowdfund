# DEMO status
The smart contract has some core logic test coverage, more tests to be added later as we progress, but the implementation is 95%+ ready to be used.

The UI still needs some work, there'll be some features that will be mocked (or covered with a "Coming soon" message).

The microservices/backend layer is work in progress, we only rely on the AWS lambda and public api/gateway at the moment.

## DEMO Steps
First of all, you'll need a MultiversX wallet with some devnet EGLD on it.
Head over to https://devnet-wallet.multiversx.com to create your wallet and get some test tokens.

- Go to https://mvp.refracto.io
- Connect with a MultiversX wallet
- Navigate to `profile` page
- Click on `Mock Successful KYC` button and send the transaction. This will be replaced by a full KYC procedure in the future
- Click on `Mint test USDC` which will yield you 20,000 RUSDC (Refracto devnet USDC - an USDC mockup for this phase)

At this point, your wallet has a successful KYC and 20,000 spare test dollars.
You can:
- navigate existing projects and invest in them
- see your account overview (which includes USDC balance, share balance, expected revenue and more) and portfolio
- withdraw from one of your investments (to be fully compliant, each investor has a 14 days window of withdrawing his investment)


# refracto-sc-crowdfund
There are 3 contracts in this repo:
- demo-usdc-faucet: an ESDT minter that allows anyone to mint tokens freely. Only used on devnet to supply an USDC mock
- loan-crowdfund-sc: the main crowdfunding logic. This is where all the projects and core logic reside
- loan-refund-escrow-sc: the repayment contract deployed when creating a project. Used for loan repayments only

# UI

Here's the link to our UI repository: https://github.com/alexbolog/refracto-app

# AWS Lambda

Instead of relying on the default native auth token, we're supplying a DB generated message on login.
Once the user connects, the dApp posts the data to an AWS Lambda instance that checks the validity of the message and the signature, making sure the connected wallet address is indeed the one we're expecting.
On a successful validation, the dApp receives a session ID and a refresh token that further represent the user's access to the supabase DB.

# DB - Supabase
We're using Supabase as a DB (PostgreSQL). Each user's data is protected through RLS (Row Level Security), which, unless the user manages to connect with/feed a different wallet address than his, means it won't be able to access any other data that's not being bound to his username.

# Mentions
Some of the RLS policies and SC permissions have been lifted/disabled for demo purposes.
