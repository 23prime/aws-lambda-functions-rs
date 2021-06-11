FROM public.ecr.aws/lambda/provided:al2

COPY ./target/release/notification-to-msteams ${LAMBDA_RUNTIME_DIR}/bootstrap

CMD [ "lambda-handler" ]
