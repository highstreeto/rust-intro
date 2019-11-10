$ErrorActionPreference = "Stop"

$gitInfo = Get-GitInfo
$gitLastCommit = $gitInfo.Tip

if ("master" -ne $gitInfo.Branch) {
    Write-Error "Must be on master branch to release!"
}

Write-Output "Building for packaging ..."

Push-Location .\slides

npm run package
Move-Item rust-intro.zip ..\

Pop-Location

Write-Output "Done!"

Write-Output "Stashing and switching to gh-pages ..."
git stash
git checkout gh-pages
Write-Output "Done!"

Write-Output "Expanding build artifact ..."
Expand-Archive -Path rust-intro.zip -DestinationPath .
Remove-Item rust-intro.zip
Write-Output "Done!"

Write-Output "Commiting and pushing (based on $gitLastCommit) ..."
git add *
git commit --all -m "Update gh-pages to commit $gitLastCommit"
git push
Write-Output "Done!"

Write-Output "Switching back to master and popping stash ..."
git checkout master
git stash pop
Write-Output "Done!"