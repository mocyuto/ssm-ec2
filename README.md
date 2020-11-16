# ec2-search ![Test](https://github.com/mocyuto/ec2-search/workflows/Test/badge.svg?branch=master)

Search EC2 instance easily

## Installation

### Cargo Install

With Rust's package manager cargo, you can install via:
```shell script
$ cargo install ec2-search
```
If you install the latest master branch commit
```shell script
$ cargo install --git https://github.com/mocyuto/ec2-search --branch master
```

### Homebrew
macOS or Linux

```shell script
$ brew tap mocyuto/ec2-search
$ brew install ec2-search
```

## Usage

```shell script
$ ec2s help
```

### AWS credentials

ec2-search needs aws credentials, so you need to set credentials.
You can use Environment value or `"~/.aws/credentials"`.

For more information, see [AWS Credentials](https://github.com/rusoto/rusoto/blob/master/AWS-CREDENTIALS.md)

### Instance

Search instance info.
```shell script
$ ec2s instance help
# or alias. see help
$ ec2s i help 
```
#### info
display basic info

```shell script
$ ec2s i info -q api
ID           Name       Status   Type
i-012345678  test-api1  running  t2.micro
i-023456789  test-api2  running  t3.small
counts: 2
```


#### instance ids
display instance ids

```shell script
## like search
$ ec2s instance ids -q "api"
ID           Name
i-012345678  test-api1
i-023456789  test-api2
counts: 2

## search exact query match
$ ec2s instance ids --exq=test-api1

## search with ids
$ ec2s instance ids --ids i-abcde12345
```

#### private-ips

Display instance private IPs.

```shell script
$ ec2s instance prips -q "api"
Private IP     Name
10.0.0.1       test-api1
10.0.0.2       test-api2
counts: 2
```

### Target Group

```shell script
$ ec2s target-group help
# or alias
$ ec2s tg help
```
