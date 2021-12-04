FROM public.ecr.aws/lambda/provided:al2

COPY ./target/release/twitter-followee-list ${LAMBDA_RUNTIME_DIR}/bootstrap

CMD [ "lambda-handler" ]
