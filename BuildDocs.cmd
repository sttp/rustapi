@echo off

echo Generating docs...
cargo doc --no-deps
echo ^<meta http-equiv=^"refresh^" content=^"0; url=sttp^"^> > target\doc\index.html
echo. 2> target\doc\.nojekyll

echo Copying docs...
rmdir /s /q docs
xcopy /e /i /y target\doc docs