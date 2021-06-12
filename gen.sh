#!/bin/bash

git pull origin template:template
cargo generate --name $* --git . --branch template
