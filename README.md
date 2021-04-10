# Offchain Worker Example

This is an alternattive implementation of [OCW](https://github.com/paritytech/substrate/tree/master/frame/example-offchain-worker).

If you want to run tests in this project you have to download the whole [Substrate](https://github.com/paritytech/substrate) project and replace the counterpart inside Substrate with this.

This is just a skeleton of oracle for BTC/USD price, definitely not production ready at all. About OCW and related concepts please refer to [official Substrate documents](https://substrate.dev/docs/en/knowledgebase/learn-substrate/off-chain-features).

Roughly, this project grabs some addresses via Substrate builtin HTTP module, and use builtin lite_json module plus lite_json_2_map module which I developed to deserialize JSON data into HashMap as OCW local storage and submit the smart contract in the end.

