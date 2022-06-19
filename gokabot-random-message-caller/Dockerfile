FROM public.ecr.aws/lambda/provided:al2

COPY ./docker-build-target/release/gokabot-random-message-caller ${LAMBDA_RUNTIME_DIR}/bootstrap

CMD [ "lambda-handler" ]
