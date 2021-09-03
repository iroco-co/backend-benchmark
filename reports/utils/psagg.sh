#!/bin/bash

PROCESS_NAME=$1

ps -C $PROCESS_NAME -o %cpu,time --no-headers | awk '{s+=$1} END {print s,$2}'
