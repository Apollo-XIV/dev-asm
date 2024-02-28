
provider "aws" {
  region                   = "eu-west-1"
  profile                  = "default"
  shared_credentials_files = "$HOME/.aws/credentials"
}

