# Shimmer

A very simple tool to make shims out of an existing executable. Only works
on Windows.

I use this to create wrapper executables so I can avoid adding things into
my `%PATH%`.

## Usage

```
shm.exe <source> <target>
```

The syntax is loosely based on `ln`. You can use `--help` to learn more.

## Architecture

The project consists of two parts. `shim` produces a `shim.exe`. It cannot be
used on its own, but serves as a template. You need to put it somewhere.

`shimmer` produces `shm.exe` that is the actual entry point. It need to be
compiled with a compile-time environment variable `SHIM_DIR` that indicates
where to find the previously compiled `shim.exe` template. If not given,
`shm.exe` will look for `shim.exe` in the same directory it resides.

### Build examples

In-place debugging:

```
> set SHIM_DIR=
> cargo build  % for debug; no need to specify SHIM_DIR; the default is good.
```

For actual use:

```
> set SHIM_DIR=%USERPROFILE%\AppData\Local\shimmer
> cargo build --release
> copy target\release\shim.exe %SHIM_DIR%
```

