# Voting service.
This project is simple voting service based on Exonum framework.
This project implements operations:
- Create a new elector.
- Create a new candidate.
- Vote for the candidate.
- Get voting results.

## Install and run
Simply run the following command to start the voting service on 2 nodes
on the local machine.

Clone the project:
```sh
git clone https://github.com/RIg410/voting_service.git
```
Build the project:
```sh
cd voting_service
cargo install
```
Generate template:
```sh
mkdir service
voting_service generate-template service/common.toml --validators-count 2
```
Generate public and secrets keys for each node:
```sh
voting_service generate-config service/common.toml  service/pub_1.toml service/sec_1.toml --peer-address 127.0.0.1:6331
voting_service generate-config service/common.toml  service/pub_2.toml service/sec_2.toml --peer-address 127.0.0.1:6332
```

Finalize configs:
```sh
voting_service finalize --public-api-address 0.0.0.0:8200 --private-api-address 0.0.0.0:8091 service/sec_1.toml service/node_1_cfg.toml --public-configs service/pub_1.toml service/pub_2.toml 
voting_service finalize --public-api-address 0.0.0.0:8201 --private-api-address 0.0.0.0:8092 service/sec_2.toml service/node_2_cfg.toml --public-configs service/pub_1.toml service/pub_2.toml
```

Run nodes:
```sh
voting_service run --node-config service/node_1_cfg.toml --db-path service/db1 --public-api-address 0.0.0.0:8200
voting_service run --node-config service/node_2_cfg.toml --db-path service/db2 --public-api-address 0.0.0.0:8201
```

#Api
Base URL for voting service endpoints: http://{host}:{port}/api/services/voting

- Create a new elector.
```
    POST v1/elector
    
    {
      "body": {
        "name": "Den lee",
        "pub_key": "c794e65c36982969b8dead7c0255bfbf13fdfca2eb75a983f2685c2d7a834361"
      },
      "message_id": 1,
      "protocol_version": 0,
      "service_id": 13,
      "signature": "669d7f3061e04c943dd079a818183ccccc94a7c88d0f450f95cec916cdfc4120366cd3b57071945d31371b34440451aaf14ffc9e299c56335106ccae2fe6350d"
    }
```
```
name is string with the owner's name.
pub_key public key of the elector.
message_id is message type.
protocol_version is the major version of the Exonum serialization protocol. Currently, 0.
service_id is service id. Voting service id is 13:)
signature is Ed25519 digital signature.
```
Returns the hex-encoded hash of the transaction encumbered in an object: `{ "tx_hash": <hash> }`.

- Create a new candidate.

```
    POST v1/candidate
    
    {
      "body": {
        "name": "John Forbes Nash",
        "pub_key": "cce2740e13bd81d1ef1b631139bf6b89755ef01cab07cee000ccb3766b4e77d4"
      },
      "message_id": 0,
      "protocol_version": 0,
      "service_id": 13,
      "signature": "bb2432da7f8230ccd4e1f8588e4e6b11ee6006c8d76f680752615029dcbb8c90ec9312ffc87bde375de33152a2240016cf91bbb912ffc4954847beb1b87ecf06"
    }
```
```  
name is string with the owner's name.
pub_key public key of the candidate.
```
Returns the hex-encoded hash of the transaction encumbered in an object: `{ "tx_hash": <hash> }`.

- Vote for the candidate.

```
    POST v1/vote
    
    {
      "body": {
        "candidate": "cce2740e13bd81d1ef1b631139bf6b89755ef01cab07cee000ccb3766b4e77d4",
        "elector": "c794e65c36982969b8dead7c0255bfbf13fdfca2eb75a983f2685c2d7a834361"
      },
      "message_id": 2,
      "protocol_version": 0,
      "service_id": 13,
      "signature": "9a36e3d88589eebbb51988241e470bd8f0d7e26c00e1600b650b034e5493a8f53ae64ba3fd13e257812ba44120af64b77222636a885903052ee96e1d1238f408"
    }
```

```
   candidate is the public key of the candidate.
   elector is the public key of the elector.
```
Returns the hex-encoded hash of the transaction encumbered in an object: `{ "tx_hash": <hash> }`.

- Gets block number by elector public key.
```
GET v1/vote/block?pub_key=c794e65c36982969b8dead7c0255bfbf13fdfca2eb75a983f2685c2d7a834361
```
Returns block number.

- Get voting results with proof.
```
GET v1/results
```
Returns json with voting results and all proofs.
```
{
  "candidates": [
    {
      "candidate": {
        "history_hash": "74dcdb089c96b0b12ce483ec940d8b6d34c48921f3a504348060ff7e19de885c",
        "name": "John Forbes Nash",
        "pub_key": "cce2740e13bd81d1ef1b631139bf6b89755ef01cab07cee000ccb3766b4e77d4",
        "voices": "1"
      },
      "vote_percent": 100.0,
      "proof": {
        "entries": [
          {
            "key": "cce2740e13bd81d1ef1b631139bf6b89755ef01cab07cee000ccb3766b4e77d4",
            "value": {
              "history_hash": "74dcdb089c96b0b12ce483ec940d8b6d34c48921f3a504348060ff7e19de885c",
              "name": "John Forbes Nash",
              "pub_key": "cce2740e13bd81d1ef1b631139bf6b89755ef01cab07cee000ccb3766b4e77d4",
              "voices": "1"
            }
          }
        ],
        "proof": []
      },
      "history": {
        "transactions": [
          {
            "body": {
              "name": "John Forbes Nash",
              "pub_key": "cce2740e13bd81d1ef1b631139bf6b89755ef01cab07cee000ccb3766b4e77d4"
            },
            "message_id": 0,
            "protocol_version": 0,
            "service_id": 13,
            "signature": "bb2432da7f8230ccd4e1f8588e4e6b11ee6006c8d76f680752615029dcbb8c90ec9312ffc87bde375de33152a2240016cf91bbb912ffc4954847beb1b87ecf06"
          },
          {
            "body": {
              "candidate": "cce2740e13bd81d1ef1b631139bf6b89755ef01cab07cee000ccb3766b4e77d4",
              "elector": "c794e65c36982969b8dead7c0255bfbf13fdfca2eb75a983f2685c2d7a834361"
            },
            "message_id": 2,
            "protocol_version": 0,
            "service_id": 13,
            "signature": "9a36e3d88589eebbb51988241e470bd8f0d7e26c00e1600b650b034e5493a8f53ae64ba3fd13e257812ba44120af64b77222636a885903052ee96e1d1238f408"
          }
        ],
        "history_proof": {
          "left": {
            "val": "2969a2670498198a8d64a3b3436cf50a26efb0907cb2eef9be0a1be53af78cf6"
          },
          "right": {
            "val": "e81a827d1052aeb7c81d8b7f591f58de7a035b0987c843baa2ad908fa190107e"
          }
        }
      }
    }
  ],
  "block_proof": {
    "block": {
      "height": "27725",
      "prev_hash": "bf7e65d283c14d3bef518572ceb532e3a6625cb7ef5be2782e1b114415618429",
      "proposer_id": 0,
      "state_hash": "b1db523ac804535de9d4efdb7cb9070609ff4dac269ded40ebcf1d62520f033b",
      "tx_count": 0,
      "tx_hash": "0000000000000000000000000000000000000000000000000000000000000000"
    },
    "precommits": [
      {
        "body": {
          "block_hash": "3251a8a77051735b479c92262f9f654e6afc815e4c28d3b19035112271bb319d",
          "height": "27725",
          "propose_hash": "06e2d06b6422edd9e15e84b1c69dd4c7644f3568818df85581f75e36608940c3",
          "round": 1,
          "time": {
            "nanos": 144942000,
            "secs": "1532912031"
          },
          "validator": 0
        },
        "message_id": 4,
        "protocol_version": 0,
        "service_id": 0,
        "signature": "29e8aff601e8bedc0ac3e0fc5003a248c42f3c72080f78d8f4fd0bbed991a72244973f58bc2aa29be72819d5008cc96e8ebbd8cb74aeb36099f943e7f068b200"
      },
      {
        "body": {
          "block_hash": "3251a8a77051735b479c92262f9f654e6afc815e4c28d3b19035112271bb319d",
          "height": "27725",
          "propose_hash": "06e2d06b6422edd9e15e84b1c69dd4c7644f3568818df85581f75e36608940c3",
          "round": 1,
          "time": {
            "nanos": 144696000,
            "secs": "1532912031"
          },
          "validator": 1
        },
        "message_id": 4,
        "protocol_version": 0,
        "service_id": 0,
        "signature": "cb63e0776fd11991f56907d429fe9c0f6b9cb49ca10a4cb1cc7df980412deb1f69653d80940e33cf2aa9f681aa13fc185748eaded2b2beaed57634b86e241602"
      }
    ]
  },
  "to_table": {
    "entries": [
      {
        "key": "43c66c260828c9839f26474151db105481ff92f5e01377f75389d4ce3d2dd574",
        "value": "39dba78e8cde08f39a7d69b578aeb9d15d30f07d627ea5a3df7cb16dd582e4d2"
      }
    ],
    "proof": [
      {
        "path": "0000101010101110110000001010110110011000000001100011001110110111000101011001101100100100000010011111001000011101110010101110111001111111101111101110100011111110000111011111101111110011011010100100110101110010101000101110101000100110011100100010101101100001",
        "hash": "0000000000000000000000000000000000000000000000000000000000000000"
      },
      {
        "path": "1101011001111000110011100000010110010000110100101101111010000001101101001101110001111001010001001111001111011111111100100100000110011100100011000001100111100010010010100011010011110010100001011100000010101110110111001001011000001010010000001111000011001001",
        "hash": "08a6df4ea96731e420d7a044d8d0c32d591ff51d5939ac81c5a90a1bbe7810e1"
      },
      {
        "path": "111",
        "hash": "00defb08107f5d76ba335c8460332f80dd399946943de10e1ea17f72cf8e4f8c"
      }
    ]
  }
}
```
- Gets elector by public key.
```
GET v1/elector?pub_key=c794e65c36982969b8dead7c0255bfbf13fdfca2eb75a983f2685c2d7a834361
```
Returns json array elector.
```
{
  "has_vote": true,
  "name": "Den lee",
  "pub_key": "c794e65c36982969b8dead7c0255bfbf13fdfca2eb75a983f2685c2d7a834361"
}
```

- Gets candidate by public key.
```
GET v1/candidate?pub_key=cce2740e13bd81d1ef1b631139bf6b89755ef01cab07cee000ccb3766b4e77d4
```
Returns json array candidates.
```
{
  "history_hash": "2969a2670498198a8d64a3b3436cf50a26efb0907cb2eef9be0a1be53af78cf6",
  "name": "John Forbes Nash",
  "pub_key": "cce2740e13bd81d1ef1b631139bf6b89755ef01cab07cee000ccb3766b4e77d4",
  "voices": "0"
}
```

- Gets all candidates with voting result.
```
GET v1/candidates
```
Returns the json array with candidates.
```
[
  {
    "history_hash": "2969a2670498198a8d64a3b3436cf50a26efb0907cb2eef9be0a1be53af78cf6",
    "name": "John Forbes Nash",
    "pub_key": "cce2740e13bd81d1ef1b631139bf6b89755ef01cab07cee000ccb3766b4e77d4",
    "voices": "0"
  }
]
```