# playground

In this repo you can find some useful files to run the provided references
binaries and see the expected behavior of the `asm` and `vm` binaries.

## Requirements

To run the provided script to test the reference binaries you need to have
installed [docker](https://docs.docker.com/get-docker/) on your computer.

## How to use this directory

Run the provided script `./ssh_playground.sh` to run a docker container where
you will be able to run the reference binaries (`asm_ref` and `vm_ref`) to test
the behavior of the _assembler_ and _VM_.

In the directory `/players_src/`, there are some player source code that can be
used as a test for your assembler binary.

The directory `/solution/` is accessible within the Docker container and all
the changes made here, from the docker container and outside, will remain
persistent.
