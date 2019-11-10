Push-Location .\slides

npm run package
Move-Item rust-intro.zip ..\

Pop-Location

Expand-Archive -Path rust-intro.zip -DestinationPath .
Remove-Item rust-intro.zip