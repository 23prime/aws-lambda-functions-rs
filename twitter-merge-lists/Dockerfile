FROM public.ecr.aws/lambda/provided:al2

COPY ./docker-build-target/release/twitter-merge-lists ${LAMBDA_RUNTIME_DIR}/bootstrap

CMD [ "lambda-handler" ]
