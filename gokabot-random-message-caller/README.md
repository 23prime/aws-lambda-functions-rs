# gokabot-random-message-caller #

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
