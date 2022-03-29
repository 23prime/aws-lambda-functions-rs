FROM public.ecr.aws/lambda/provided:al2

COPY ./target/release/twitter-merge-lists ${LAMBDA_RUNTIME_DIR}/bootstrap

CMD [ "lambda-handler" ]
