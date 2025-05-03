#!/bin/sh

read -p "Enter AWS profile : " aws_profile
aws_profile=${aws_profile:-personal}
aws-vault exec $aws_profile -- env | grep -e AWS_REGION -e AWS_DEFAULT_REGION -e AWS_ACCESS_KEY_ID -e AWS_SECRET_ACCESS_KEY -e AWS_SESSION_TOKEN
