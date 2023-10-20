# BLUMER-MS-PROFILE


The MS is responsible for handling all user profile related tasks. This includes manage user profiles, updating existing profiles, and deleting profiles when requested by the user. The MS also handles tasks such as profile privacy settings, and profile visibility settings. It provides Async events to other microservices to access user profile data, ensuring that profile data is consistent across the distributed system. It also handles user authentication and authorization, ensuring that only authorized users can access profile data. 

## Features

- Get Profile info
  - By authenticated user
  - By Id
  - By username 
- Edit Profile description
- Edit profile names
- Edit profile username
- Edit privacy


## Events


## Prerequisites
- Rust [install guide](https://www.rust-lang.org/es/tools/install)
- CapnProto [install guide](https://capnproto.org/install.html)
- GRPC [install guide]()
- Openssl

## Installation

Simply go over the following checklist:

1. Create a `deps` folder.
2. Git clone the following dependencies or libs inside `deps`.
- [blumer-lib-errors](https://github.com/blumerapi/blumer-lib-errors)
- [blumer-lib-auth-rs](https://github.com/blumerapi/blumer-lib-auth-rs)
- [blumer-lib-authorization-rs](https://github.com/blumerapi/blumer-lib-authorization-rs)


## Building

Once the prerequisites have been installed, compilation on your native platform is as simple as running the following in a terminal:

```
cargo build --release
```
## Tech Stack

- Rust[![Actix-Web](https://avatars.githubusercontent.com/u/5430905?s=48&v=4)](https://github.com/rust-lang/rust)
- Actix Web[![Actix-Web](https://avatars.githubusercontent.com/u/32776943?s=48&v=4)](https://github.com/actix/actix-web)
- Async Graphql[![AsyncGraphql](https://avatars.githubusercontent.com/u/12972006?s=48&v=4)](https://github.com/async-graphql/async-graphql)
- Redis[![Redis](https://avatars.githubusercontent.com/u/1529926?s=48&v=4)](https://github.com/redis/redis)
- ScyllaDB[![ScyllaDB](https://avatars.githubusercontent.com/u/14364730?s=48&v=4)](https://github.com/scylladb/scylladb)
- Kafka[![Kafka](https://kafka.apache.org/logos/kafka_logo--simple.png)](https://github.com/apache/kafka)
- Cap'n Proto[![Cap'n Proto](https://avatars.githubusercontent.com/u/29186932?s=48&v=4)](https://github.com/capnproto)

## API Reference Graphql

#### profileByID 
Get a profile by id or logged in profile
```graphql
query ProfileById {
  profileById {
    id
    username
    names
    photo
    photoHash
    portrait
    portraitHash
    description
    verified
    privacy
    **other extensions**
  }
}
```

| Parameter | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `id` | `ID` | **Optional**. profile ID |

#### profileByUsername
Get a profile by username
```graphql
query ProfileByUsername($username: String!) {
  profileByUsername(username: $username) {
    id
    username
    names
    photo
    photoHash
    portrait
    portraitHash
    description
    verified
    privacy
    **other extensions**
  }
}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `username`      | `string` | **Required**. profile username |



#### profileEditDescription
For edit an profile description

```graphql
mutation ProfileEditDescription($description: String!) {
  profileEditDescription(description: $description)
}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `description`      | `string` | **Required**. profile description or caption |


#### profileEditNames
For edit names, only can be edited each 30 days

```graphql
mutation ProfileEditNames($names: String!) {
  profileEditNames(names: $names)
}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `names`      | `string` | **Required**. profile names 30ch |



#### profileEditPrivacy
For edit privacy

```graphql
mutation ProfileEditPrivacy($privacy: Boolean!) {
  profileEditPrivacy(privacy: $privacy)
}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `privacy`      | `boolean` | **Required**. profile privacy |



#### profileEditUsername
For edit username, only can be edited each 30 days

```graphql
mutation ProfileEditUsername($username: String!) {
  profileEditUsername(username: $username)
}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `username`      | `string` | **Required**. profile username min 4 characters max 20 |


## Environment Variables

To run this project, you will need to add the following environment variables to your .env file you can use it from .sample-env

`GRAPHQL_HOST` the host an port where will run the graphql server 0.0.0.0:5002

`KAFKA_GROUP` Kafka group for receive messages 

`KAFKA_BROKER` The kafka server url, if you are using a cluster create a string separted by comma for each cluster node, example "node1,node2,node3"

`KAFKA_USER` Kafka cluster user

`KAFKA_PASSWORD` Kafka cluster password

`KAFKA_MECHANISMS` generally SCRAM-SHA-512

`SCYLLADB_USER` scylladb user

`SCYLLADB_PASSWORD` scylladb password

`SCYLLADB_NODES` The scylladb server url, if you are using a cluster create a string separted by comma for each cluster node, example "node1,node2,node3"

`AWS_CLOUDFRONT_PK` CloudFront private Key file location example relative path files/profile_private_key.pem

`AWS_CLOUDFRONT_ID` Cloudfront Distribution ID

`AWS_CLOUDFRONT_DOMAIN` Cloudfront Distribution domain

`REDIS_URL` REDIS url

`AUTH_URL` Authorization GRPC server URL
## Running Tests

To run tests, run the following command

```bash
  cargo test
```


## Run Locally

Clone the project

```bash
  git clone https://github.com/blumerapi/blumer-ms-profile
```

Go to the project directory

```bash
  cd blumer-ms-profile
```

Complete installation step

Run the project

```bash
  cargo run
```


## Deployment

To deploy this project run

```bash
  docker compose -f docker-compose-prod.yml up -d
```