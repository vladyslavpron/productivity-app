SET "RUST_ENV=PRODUCTION"
@REM PATH MUST BE RELATIVE TO BATCH FILE
SET "STATIC_FOLDER=./static/"

start .\dist\service\service.exe
start .\dist\my_electron_app.exe