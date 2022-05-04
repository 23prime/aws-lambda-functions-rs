# tonarinoyj-update-checker #

Check update a content of [となりのヤングジャンプ](https://tonarinoyj.jp/).

## Migration ##

### Install sqlx-cli ###

TODO: Use in Docker.

```console
$ cargo install --version=0.5.13 sqlx-cli --no-default-features --features native-tls,postgres
```

### Migration commands ###

```console
$ export DATABASE_URL=...
$ sqlx migrate add create_latest_entries
$ sqlx migrate info --source migrations
$ sqlx migrate run
```

## Build ##

```console
$ ./build.sh
```

## Deploy ##

```console
$ ./deploy.sh
```

## Test at local ##

You can test on your local PC by using [RIE](https://docs.aws.amazon.com/lambda/latest/dg/images-test.html).

Copy `.env.template` to `.env` and set your env vars.

```console
$ cp .env.template .env
```

Build a image and run container.

```console
$ ./build.sh
$ docker-compose up
```

Call Lambda endpoint with a sample CloudWatch scheduled event.

```console
$ curl -X POST "http://localhost:9000/2015-03-31/functions/function/invocations" -d @./test/events/cloudwatch-scheduled-event.json
```
