# twitter-merge-lists #

Merge some Twitter lists into one list.

## Env vars ##

See `.env.template`.

- `SOURCE_LISTS`
  - Set source list IDs with comma separated.
  - eg.) `SOURCE_LISTS=123456,234567,345678`
- `TARGET_LIST`
  - Set your just one list ID.
  - eg.) `TARGET_LIST=987654`

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
$ curl -XPOST "http://localhost:9000/2015-03-31/functions/function/invocations" -d @./test/events/cloudwatch-scheduled-event.json
```
