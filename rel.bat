docker image tag rsepp heartleth/enpp-rust:latest
docker push heartleth/enpp-rust:latest
git add .
set /p dl=Commit message: 
git commit -m "%dl%"