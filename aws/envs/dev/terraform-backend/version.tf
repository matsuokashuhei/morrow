terraform {
  required_version = "1.11.4"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "5.97.0"
    }
    awscc = {
      source  = "hashicorp/awscc"
      version = "1.39.0"
    }
  }
}
