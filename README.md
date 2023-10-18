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
