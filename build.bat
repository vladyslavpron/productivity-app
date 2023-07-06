if not exist ".\dist\" mkdir ".\dist"

@REM Build and copy Electron app
cd ./app
call npm run package
cd ..
copy ".\app\dist\my_electron_app.exe" ".\dist\"
robocopy ".\app\build\static" ".\dist\static" /E


@REM Build and copy service
cd ./service
cargo build --release
cd ..
copy ".\service\target\release\service.exe" ".\dist\service\"

copy "start.bat" ".\dist\"