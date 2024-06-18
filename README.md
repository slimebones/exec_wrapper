# exec_wrapper

The command:
```sh
exec_wrapper "python hello.py" hello
```
will create an executable `hello` in the current dir, which on invoke, will
call `python hello.py`. It also accepts arguments and appends them to the
final command, e.g.:
```sh
hello arg1 arg2 arg3
```
will result in:
```sh
python hello.py arg1 arg2 arg3
```
