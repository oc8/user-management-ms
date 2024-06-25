#!/bin/bash
sleep 5
sqlx migrate run
./user-service