FROM public.ecr.aws/lambda/provided:al2

COPY ./target/release/notification-by-gokabot ${LAMBDA_RUNTIME_DIR}/bootstrap

CMD [ "lambda-handler" ]
